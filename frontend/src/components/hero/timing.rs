//! When the slideshow moves: each slide holds for its time, then slides on to the next in
//! a moment. Written as a CSS `linear()` easing over one whole cycle, so the slides run on
//! a single Tailwind animation and need no script.

/// How long each slide shows, in milliseconds.
pub const SLIDE_MS: u32 = 8_000;
/// How long the move to the next slide takes, in milliseconds.
const MOVE_MS: u32 = 350;

/// The easing that walks through `stops` positions, one per slide: it holds each, then
/// moves on at the end of that slide's time. With `wrap`, the last slide moves back to
/// the first; without, it moves on to one more position (a copy of the first slide), so
/// the cycle restarting there is seamless.
pub fn ease(slides: usize, wrap: bool) -> String {
    let n = slides.max(1) as f64;
    let positions = if wrap { n } else { n + 1.0 };
    let moving = f64::from(MOVE_MS) / (n * f64::from(SLIDE_MS));
    let mut points = vec!["0 0%".to_owned()];
    for k in 0..slides {
        let (from, end) = (k as f64 / positions, (k + 1) as f64 / n);
        let to = if wrap && k + 1 == slides {
            0.0
        } else {
            (k + 1) as f64 / positions
        };
        points.push(format!(
            "{} {}%",
            round(from),
            round((end - moving) * 100.0)
        ));
        points.push(format!("{} {}%", round(to), round(end * 100.0)));
    }
    format!("linear({})", points.join(", "))
}

/// How long a slide's text takes to fade in or out, in milliseconds.
const FADE_MS: f64 = 200.0;
/// How long a line of text takes to slide in, in milliseconds.
const ENTER_MS: f64 = 500.0;

/// When slide `k`'s turn starts: as the photo before it starts moving off. The first
/// slide's turn starts as the cycle ends, while the copy of it slides in.
fn turn(k: usize, slides: usize) -> f64 {
    let cycle = slides as f64 * f64::from(SLIDE_MS);
    (k as f64 * f64::from(SLIDE_MS) - f64::from(MOVE_MS)).rem_euclid(cycle)
}

/// Slide `k`'s text showing: it fades in on its turn and out on the next one.
pub fn shown(k: usize, slides: usize) -> String {
    if slides <= 1 {
        return "linear(1, 1)".into();
    }
    let (on, off) = (turn(k, slides), turn(k + 1, slides));
    circular(
        &[
            (on, 0.0),
            (on + FADE_MS, 1.0),
            (off, 1.0),
            (off + FADE_MS, 0.0),
        ],
        slides,
    )
}

/// Slide `k`'s bar filling: from empty on its turn to full on the next, then empty.
pub fn filling(k: usize, slides: usize) -> String {
    if slides <= 1 {
        return "linear(0, 1)".into();
    }
    let (on, off) = (turn(k, slides), turn(k + 1, slides));
    let off = if off <= on {
        off + slides as f64 * f64::from(SLIDE_MS)
    } else {
        off
    };
    circular(&[(on, 0.0), (off, 1.0), (off + 0.01, 0.0)], slides)
}

/// Slide `k`'s lines arriving: each slides in, easing out, on its turn, and is back in
/// place, unseen, once the text has faded out.
pub fn arriving(k: usize, slides: usize) -> String {
    if slides <= 1 {
        return "linear(1, 1)".into();
    }
    let (on, gone) = (turn(k, slides), turn(k + 1, slides) + FADE_MS);
    circular(
        &[
            (on, 0.0),
            (on + ENTER_MS * 0.25, 0.45),
            (on + ENTER_MS * 0.5, 0.75),
            (on + ENTER_MS * 0.75, 0.93),
            (on + ENTER_MS, 1.0),
            (gone, 1.0),
            (gone + 1.0, 0.0),
        ],
        slides,
    )
}

/// An easing over one cycle through `points` (milliseconds, value), which may run past
/// its end and wrap round: straight lines between them, joined up across the loop.
fn circular(points: &[(f64, f64)], slides: usize) -> String {
    let cycle = slides as f64 * f64::from(SLIDE_MS);
    let mut points: Vec<(f64, f64)> = points
        .iter()
        .map(|&(t, v)| (t.rem_euclid(cycle), v))
        .collect();
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (first, last) = (points[0], points[points.len() - 1]);
    let span = first.0 + cycle - last.0;
    let start = if span > 0.0 {
        last.1 + (first.1 - last.1) * (cycle - last.0) / span
    } else {
        first.1
    };
    let mut out = vec![format!("{} 0%", round(start))];
    out.extend(
        points
            .iter()
            .map(|&(t, v)| format!("{} {}%", round(v), round(t / cycle * 100.0))),
    );
    out.push(format!("{} 100%", round(start)));
    format!("linear({})", out.join(", "))
}

/// How long the code on a slide takes to type out, in milliseconds.
const TYPING_MS: f64 = 4_600.0;

/// How a slide's code types out once its photo is in place: for each line with
/// something to type (after its indentation, which appears at once), the easing that
/// reveals it a character at a time, and the one that shows the cursor on it while it
/// is typed, and blinking on the last line afterwards. All are back to nothing once the
/// slide has moved off, ready for its next turn.
pub fn typing(k: usize, slides: usize, lines: &[usize]) -> Vec<Option<(String, String)>> {
    let slide = f64::from(SLIDE_MS);
    let start = k as f64 * slide + 400.0;
    let reset = (k + 1) as f64 * slide + 60.0;

    // A typist's uneven rhythm, the same on every render: a little pause per line.
    let mut seed = 0x9E37_79B9_u32.wrapping_mul(k as u32 + 1);
    let mut jitter = move || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        0.6 + f64::from(seed % 1000) / 1000.0 * 0.8
    };
    let mut times: Vec<Vec<f64>> = Vec::new();
    let mut clock = 0.0;
    for &length in lines {
        clock += 4.0;
        times.push(
            (0..length)
                .map(|_| {
                    clock += jitter();
                    clock
                })
                .collect(),
        );
    }
    let scale = if clock > 0.0 { TYPING_MS / clock } else { 0.0 };
    let last = lines.iter().rposition(|&length| length > 0);

    times
        .iter()
        .enumerate()
        .map(|(j, chars)| {
            let length = chars.len();
            if length == 0 {
                return None;
            }
            let at = |t: f64| start + t * scale;
            let mut reveal = Vec::with_capacity(length * 2 + 2);
            for (i, &t) in chars.iter().enumerate() {
                reveal.push((at(t), i as f64 / length as f64));
                reveal.push((at(t) + 0.01, (i + 1) as f64 / length as f64));
            }
            reveal.push((reset, 1.0));
            reveal.push((reset + 0.01, 0.0));

            let (from, to) = (at(chars[0]) - 60.0, at(chars[length - 1]));
            let mut cursor = vec![(from - 0.01, 0.0), (from, 1.0)];
            if Some(j) == last {
                // Blinking until the slide moves on.
                let mut t = to;
                let mut on = true;
                while t + 500.0 < reset - 400.0 {
                    t += 500.0;
                    on = !on;
                    cursor.push((t - 0.01, if on { 0.0 } else { 1.0 }));
                    cursor.push((t, if on { 1.0 } else { 0.0 }));
                }
                cursor.push((t + 0.01, 0.0));
            } else {
                let next = times[j + 1..].iter().find_map(|line| line.first().copied());
                let until = next.map_or(to, |t| at(t) - 60.0);
                cursor.push((until, 1.0));
                cursor.push((until + 0.01, 0.0));
            }
            Some((circular(&reveal, slides), circular(&cursor, slides)))
        })
        .collect()
}

fn round(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::ease;

    #[test]
    fn holds_each_slide_then_moves_on_to_the_copy_of_the_first() {
        assert_eq!(
            ease(2, false),
            "linear(0 0%, 0 47.8125%, 0.3333 50%, 0.3333 97.8125%, 0.6667 100%)"
        );
    }

    #[test]
    fn the_first_slides_text_shows_until_the_seconds_turn() {
        // Two slides make a 16s cycle: the second's turn starts at 7.65s, the first's
        // again at 15.65s, as the copy of it slides in.
        assert_eq!(
            super::shown(0, 2),
            "linear(1 0%, 1 47.8125%, 0 49.0625%, 0 97.8125%, 1 99.0625%, 1 100%)"
        );
    }

    #[test]
    fn wrapping_moves_the_last_back_to_the_first() {
        assert_eq!(
            ease(2, true),
            "linear(0 0%, 0 47.8125%, 0.5 50%, 0.5 97.8125%, 0 100%)"
        );
    }
}
