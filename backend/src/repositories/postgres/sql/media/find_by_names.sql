-- The files with these names.
SELECT media.id, media.file_name, media.content_type
FROM media
WHERE media.file_name = ANY($1)
