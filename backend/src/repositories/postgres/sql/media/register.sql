-- A file found in the media folder, unless it is already known.
INSERT INTO media (id, file_name, content_type, size_bytes)
VALUES ($1, $2, $3, $4)
ON CONFLICT (file_name) DO NOTHING
