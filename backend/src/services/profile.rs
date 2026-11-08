//! What the About page shows: the stack, skills and work history.

use super::cache::{UNTIL_CHANGED, cached};
use crate::{
    models::{Experience, Skill, StackItem},
    repositories::postgres::profile,
    repositories::redis::keys,
    state::AppState,
};

pub async fn stack(state: &AppState) -> anyhow::Result<Vec<StackItem>> {
    cached(state, keys::STACK, UNTIL_CHANGED, async {
        Ok(profile::stack(&state.pool).await?)
    })
    .await
}

pub async fn skills(state: &AppState) -> anyhow::Result<Vec<Skill>> {
    cached(state, keys::SKILLS, UNTIL_CHANGED, async {
        Ok(profile::skills(&state.pool).await?)
    })
    .await
}

pub async fn experience(state: &AppState) -> anyhow::Result<Vec<Experience>> {
    cached(state, keys::EXPERIENCE, UNTIL_CHANGED, async {
        Ok(profile::experience(&state.pool).await?)
    })
    .await
}
