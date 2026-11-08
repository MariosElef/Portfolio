-- A project already there gains a logo or cover it is missing.
UPDATE projects
SET
    logo_id = coalesce(projects.logo_id, (SELECT media.id FROM media WHERE media.file_name = $2)),
    cover_id = coalesce(projects.cover_id, (SELECT media.id FROM media WHERE media.file_name = $3))
WHERE projects.slug = $1 AND (projects.logo_id IS NULL OR projects.cover_id IS NULL)
