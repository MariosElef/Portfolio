//! A small sum the contact form asks, so a person, not a script, sends each message.
//!
//! The answer is never sent to the browser or stored: the token carries a random id and
//! when it expires, signed together with the answer under the server's key. An answer is
//! right when it makes the same signature, before the token expires; then the id is
//! spent in Redis, so the same question cannot be answered twice.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, KeyInit, Mac};
use rand::Rng;
use sha2::Sha256;

use crate::{models::Challenge, repositories::redis::keys, state::AppState};

/// How long a question can be answered.
const LIFETIME_SECONDS: u64 = 30 * 60;

const ID: usize = 16;
const EXPIRES: usize = 8;
const SIGNATURE: usize = 32;

/// A new question.
pub fn ask(key: &[u8]) -> Challenge {
    let mut rng = rand::rng();
    let (a, b) = (rng.random_range(2..=9), rng.random_range(2..=9));
    let id: [u8; ID] = rng.random();
    let expires = now() + LIFETIME_SECONDS;
    Challenge {
        a,
        b,
        token: token(key, &id, expires, u32::from(a) + u32::from(b)),
    }
}

/// Whether `answer` answers the question in `token`, for the first time.
pub async fn answered(state: &AppState, token: &str, answer: &str) -> bool {
    let Some((id, expires)) = check(&state.challenge_key, token, answer, now()) else {
        return false;
    };
    let key = keys::challenge(&hex(&id));
    // Without Redis, an answer cannot be spent; it is let through rather than lost.
    state
        .redis
        .spend(&key, expires.saturating_sub(now()).max(1))
        .await
        .unwrap_or(true)
}

fn token(key: &[u8], id: &[u8; ID], expires: u64, answer: u32) -> String {
    let mut bytes = Vec::with_capacity(ID + EXPIRES + SIGNATURE);
    bytes.extend_from_slice(id);
    bytes.extend_from_slice(&expires.to_be_bytes());
    bytes.extend_from_slice(&signature(key, id, expires, answer).finalize().into_bytes());
    URL_SAFE_NO_PAD.encode(bytes)
}

fn signature(key: &[u8], id: &[u8], expires: u64, answer: u32) -> Hmac<Sha256> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC takes a key of any length");
    mac.update(id);
    mac.update(&expires.to_be_bytes());
    mac.update(&answer.to_be_bytes());
    mac
}

/// The token's id and expiry, when `answer` is right and it has not expired.
fn check(key: &[u8], token: &str, answer: &str, now: u64) -> Option<([u8; ID], u64)> {
    let answer: u32 = answer.trim().parse().ok()?;
    let bytes = URL_SAFE_NO_PAD.decode(token.trim()).ok()?;
    if bytes.len() != ID + EXPIRES + SIGNATURE {
        return None;
    }
    let (id, rest) = bytes.split_at(ID);
    let (expires, signed) = rest.split_at(EXPIRES);
    let expires = u64::from_be_bytes(expires.try_into().ok()?);
    if expires <= now {
        return None;
    }
    // Compared in constant time.
    signature(key, id, expires, answer)
        .verify_slice(signed)
        .ok()?;
    Some((id.try_into().ok()?, expires))
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = b"test key";

    fn question() -> (Challenge, u32) {
        let challenge = ask(KEY);
        let sum = u32::from(challenge.a) + u32::from(challenge.b);
        (challenge, sum)
    }

    #[test]
    fn the_right_answer_passes() {
        let (challenge, sum) = question();
        assert!(check(KEY, &challenge.token, &format!(" {sum} "), now()).is_some());
    }

    #[test]
    fn a_wrong_answer_fails() {
        let (challenge, sum) = question();
        assert!(check(KEY, &challenge.token, &(sum + 1).to_string(), now()).is_none());
        assert!(check(KEY, &challenge.token, "eleven", now()).is_none());
    }

    #[test]
    fn an_expired_question_fails() {
        let (challenge, sum) = question();
        let later = now() + LIFETIME_SECONDS + 1;
        assert!(check(KEY, &challenge.token, &sum.to_string(), later).is_none());
    }

    #[test]
    fn another_key_or_a_changed_token_fails() {
        let (challenge, sum) = question();
        assert!(check(b"other key", &challenge.token, &sum.to_string(), now()).is_none());
        let mut changed = challenge.token.clone();
        changed.replace_range(0..1, if changed.starts_with('A') { "B" } else { "A" });
        assert!(check(KEY, &changed, &sum.to_string(), now()).is_none());
    }
}
