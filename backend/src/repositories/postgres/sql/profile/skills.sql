-- Every skill under its area, in order.
SELECT
    skill_areas.name AS area,
    skills.name,
    skills.level
FROM skills
INNER JOIN skill_areas ON skills.area_id = skill_areas.id
ORDER BY skill_areas.sort, skill_areas.name, skills.sort, skills.name
