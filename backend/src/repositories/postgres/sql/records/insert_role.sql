-- A role in the work history, unless there is one with that slug; its logo by file name.
INSERT INTO experience (
    slug, sort, year, role, company, period, summary, color, accent, logo_id, did, learned
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    (SELECT media.id FROM media WHERE media.file_name = $10),
    $11,
    $12
)
ON CONFLICT (slug) DO NOTHING
RETURNING experience.id
