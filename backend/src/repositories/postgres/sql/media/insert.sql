-- The same bytes uploaded twice keep their first id, since the file name is their hash.
INSERT INTO media (id, file_name, content_type, size_bytes)
VALUES ($1, $2, $3, $4)
ON CONFLICT (file_name) DO UPDATE SET content_type = excluded.content_type
RETURNING media.id
