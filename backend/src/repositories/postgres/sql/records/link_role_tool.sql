-- A tool used in a role, by the tool's name, at its place in the list.
INSERT INTO experience_tools (experience_id, tool_id, sort)
SELECT
    $1::int AS experience_id,
    tools.id AS tool_id,
    $3::int AS sort
FROM tools
WHERE tools.name = $2::text
ON CONFLICT DO NOTHING
