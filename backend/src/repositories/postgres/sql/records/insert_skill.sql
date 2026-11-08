-- A skill under its area by name, unless the area has it already.
INSERT INTO skills (area_id, name, level, sort)
SELECT
    skill_areas.id AS area_id,
    $2::text AS name,
    $3::smallint AS level,
    $4::int AS sort
FROM skill_areas
WHERE skill_areas.name = $1
ON CONFLICT (area_id, name) DO NOTHING
