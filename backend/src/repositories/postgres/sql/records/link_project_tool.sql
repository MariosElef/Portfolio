-- A tool a project uses, by the tool's name, at its place in the list.
INSERT INTO project_tools (project_slug, tool_id, sort)
SELECT
    $1::text AS project_slug,
    tools.id AS tool_id,
    $3::int AS sort
FROM tools
WHERE tools.name = $2::text
ON CONFLICT DO NOTHING
