-- A role already there gains a logo it is missing.
UPDATE experience
SET logo_id = (SELECT media.id FROM media WHERE media.file_name = $2)
WHERE experience.slug = $1 AND experience.logo_id IS NULL
