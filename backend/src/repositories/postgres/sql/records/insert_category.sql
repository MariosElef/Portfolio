-- A stack category, unless there is one by that name.
INSERT INTO tool_categories (name, note, sort)
VALUES ($1, $2, $3)
ON CONFLICT (name) DO NOTHING
