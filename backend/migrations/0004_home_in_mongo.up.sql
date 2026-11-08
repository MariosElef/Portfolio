-- The home page's slides are a document in MongoDB (`home`), not records: nothing links
-- to them, and they are read whole. Their content was the same as the seed's, which
-- writes that document on start.
DROP TABLE hero_slides;
