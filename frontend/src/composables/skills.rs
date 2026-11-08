use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::Skill};

/// One area of the stack experience and its skills, in order.
pub struct SkillArea {
    pub name: String,
    pub skills: Vec<Skill>,
}

/// The stack experience, grouped by area in the order the backend returns it.
pub async fn skill_areas(cx: &Cx) -> ApiResult<Vec<SkillArea>> {
    Ok(group(api(cx).skills().await?))
}

fn group(skills: Vec<Skill>) -> Vec<SkillArea> {
    let mut areas: Vec<SkillArea> = Vec::new();
    for skill in skills {
        match areas.last_mut() {
            Some(last) if last.name == skill.area => last.skills.push(skill),
            _ => areas.push(SkillArea {
                name: skill.area.clone(),
                skills: vec![skill],
            }),
        }
    }
    areas
}

#[cfg(test)]
mod tests {
    use super::group;
    use crate::types::content::Skill;

    fn skill(area: &str, name: &str) -> Skill {
        Skill {
            area: area.into(),
            name: name.into(),
            level: 3,
        }
    }

    #[test]
    fn groups_consecutive_skills_by_area() {
        let areas = group(vec![
            skill("Backend", "Rust"),
            skill("Backend", "Axum"),
            skill("Data", "SQL"),
        ]);
        let shape: Vec<_> = areas
            .iter()
            .map(|a| (a.name.as_str(), a.skills.len()))
            .collect();
        assert_eq!(shape, [("Backend", 2), ("Data", 1)]);
    }
}
