-- The work history with each role's logo and tools, newest first.
SELECT
    experience.year,
    experience.role,
    experience.company,
    experience.period,
    experience.summary,
    experience.color,
    experience.accent,
    media.file_name AS "logo_file?",
    media.content_type AS "logo_type?",
    experience.did,
    experience.learned,
    coalesce((
        SELECT array_agg(tools.name ORDER BY experience_tools.sort, tools.name)
        FROM experience_tools
        INNER JOIN tools ON experience_tools.tool_id = tools.id
        WHERE experience_tools.experience_id = experience.id
    ), '{}') AS "tags!"
FROM experience
LEFT JOIN media ON experience.logo_id = media.id
ORDER BY experience.sort, experience.id
