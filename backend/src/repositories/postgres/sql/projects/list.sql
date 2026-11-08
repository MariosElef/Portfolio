-- Every project with its tools in order, and its cover and logo files.
SELECT
    projects.slug,
    projects.initials,
    projects.tone::text AS "tone!",
    projects.name,
    projects.summary,
    coalesce((
        SELECT array_agg(tools.name ORDER BY project_tools.sort, tools.name)
        FROM project_tools
        INNER JOIN tools ON project_tools.tool_id = tools.id
        WHERE project_tools.project_slug = projects.slug
    ), '{}') AS "tags!",
    projects.repo,
    projects.featured,
    projects.availability::text AS "availability!",
    cover.file_name AS "cover_file?",
    cover.content_type AS "cover_type?",
    logo.file_name AS "logo_file?",
    logo.content_type AS "logo_type?"
FROM projects
LEFT JOIN media AS cover ON projects.cover_id = cover.id
LEFT JOIN media AS logo ON projects.logo_id = logo.id
ORDER BY projects.sort, projects.name
