-- Every project's slug.
SELECT projects.slug
FROM projects
ORDER BY projects.sort, projects.slug
