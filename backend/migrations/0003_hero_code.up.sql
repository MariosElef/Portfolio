-- A slide can show code typed out, instead of a photo or video.
ALTER TABLE hero_slides
    ADD COLUMN code text NOT NULL DEFAULT '',
    -- `rust` or `python`, for the colours.
    ADD COLUMN code_language text NOT NULL DEFAULT '';
