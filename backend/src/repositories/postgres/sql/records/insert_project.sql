-- Media is linked by file name: NULL when there is no such file.
INSERT INTO projects (
    slug, initials, tone, name, summary, repo, featured, sort, availability, logo_id, cover_id
)
VALUES (
    $1,
    $2,
    $3::text::tone,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9::text::availability,
    (SELECT media.id FROM media WHERE media.file_name = $10),
    (SELECT media.id FROM media WHERE media.file_name = $11)
)
ON CONFLICT (slug) DO NOTHING
RETURNING projects.slug
