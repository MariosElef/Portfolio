-- A skill area, unless there is one by that name.
INSERT INTO skill_areas (name, sort)
VALUES ($1, $2)
ON CONFLICT (name) DO NOTHING
