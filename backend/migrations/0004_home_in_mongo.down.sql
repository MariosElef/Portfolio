-- The slides' table as it was, empty; the seed no longer fills it.
CREATE TABLE hero_slides (
    id            int  GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    sort          int  NOT NULL DEFAULT 0,
    tab_label     text NOT NULL,
    eyebrow       text NOT NULL DEFAULT '',
    title         text NOT NULL,
    subtitle      text NOT NULL DEFAULT '',
    callout       text NOT NULL DEFAULT '',
    cta_label     text NOT NULL,
    cta_href      text NOT NULL,
    media_id      uuid REFERENCES media (id) ON DELETE SET NULL,
    poster_id     uuid REFERENCES media (id) ON DELETE SET NULL,
    code          text NOT NULL DEFAULT '',
    code_language text NOT NULL DEFAULT ''
);
CREATE TRIGGER hero_slides_changed AFTER INSERT OR UPDATE OR DELETE ON hero_slides
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
