-- A tool under its category by name, unless there is one by that name.
INSERT INTO tools (name, icon, category_id, sort)
VALUES (
    $1,
    $2,
    (SELECT tool_categories.id FROM tool_categories WHERE tool_categories.name = $3),
    $4
)
ON CONFLICT (name) DO NOTHING
