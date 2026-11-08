-- The site's records: media, projects and the tools they use, the stack, skills, work
-- history, the home page's slides, and the contact inbox. Schema only: the content the
-- site starts with is written from backend/src/seeds on start.

CREATE TYPE tone AS ENUM ('light', 'brand', 'mint', 'plum');
CREATE TYPE availability AS ENUM ('coming_soon', 'private');

-- Uploaded files, named by the sha256 of their bytes; Caddy serves them at /media.
CREATE TABLE media (
    id           uuid        PRIMARY KEY,
    file_name    text        NOT NULL CONSTRAINT media_file_name_key UNIQUE,
    content_type text        NOT NULL,
    size_bytes   bigint      NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now()
);

-- The stack: tools under their categories. A tool with no category is known (for its
-- logo) but not listed in the stack.
CREATE TABLE tool_categories (
    id   int  GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text NOT NULL CONSTRAINT tool_categories_name_key UNIQUE,
    note text NOT NULL DEFAULT '',
    sort int  NOT NULL DEFAULT 0
);

CREATE TABLE tools (
    id          int  GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name        text NOT NULL CONSTRAINT tools_name_key UNIQUE,
    -- The key of its logo in the frontend's logo map.
    icon        text NOT NULL DEFAULT '',
    category_id int  REFERENCES tool_categories (id) ON DELETE SET NULL,
    sort        int  NOT NULL DEFAULT 0
);

-- Projects, and the tools each is built with, in order.
CREATE TABLE projects (
    slug         text         PRIMARY KEY,
    initials     text         NOT NULL,
    -- The tile's colour when there is no logo.
    tone         tone         NOT NULL DEFAULT 'brand',
    name         text         NOT NULL,
    summary      text         NOT NULL,
    repo         text,
    featured     boolean      NOT NULL DEFAULT false,
    sort         int          NOT NULL DEFAULT 0,
    cover_id     uuid         REFERENCES media (id) ON DELETE SET NULL,
    logo_id      uuid         REFERENCES media (id) ON DELETE SET NULL,
    -- Why there is no link, for a project without a repo.
    availability availability NOT NULL DEFAULT 'coming_soon'
);

CREATE TABLE project_tools (
    project_slug text NOT NULL REFERENCES projects (slug) ON UPDATE CASCADE ON DELETE CASCADE,
    tool_id      int  NOT NULL REFERENCES tools (id) ON DELETE CASCADE,
    sort         int  NOT NULL DEFAULT 0,
    PRIMARY KEY (project_slug, tool_id)
);

-- The work history, and the tools used in each role.
CREATE TABLE experience (
    id      int    GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    sort    int    NOT NULL DEFAULT 0,
    role    text   NOT NULL,
    company text   NOT NULL,
    period  text   NOT NULL,
    did     text[] NOT NULL DEFAULT '{}',
    learned text[] NOT NULL DEFAULT '{}',
    year    text   NOT NULL DEFAULT '',
    summary text   NOT NULL DEFAULT '',
    color   text   NOT NULL DEFAULT '#0080f0'
        CONSTRAINT experience_color_check CHECK (color ~ '^#[0-9a-fA-F]{6}$'),
    accent  text   NOT NULL DEFAULT ''
        CONSTRAINT experience_accent_check CHECK (accent = '' OR accent ~ '^#[0-9a-fA-F]{6}$'),
    logo_id uuid   REFERENCES media (id) ON DELETE SET NULL,
    -- Stable, so a role is never found by its display name.
    slug    text   NOT NULL CONSTRAINT experience_slug_key UNIQUE
);

CREATE TABLE experience_tools (
    experience_id int NOT NULL REFERENCES experience (id) ON DELETE CASCADE,
    tool_id       int NOT NULL REFERENCES tools (id) ON DELETE CASCADE,
    sort          int NOT NULL DEFAULT 0,
    PRIMARY KEY (experience_id, tool_id)
);

-- Skills by area, each 1 to 5 stars.
CREATE TABLE skill_areas (
    id   int  GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text NOT NULL CONSTRAINT skill_areas_name_key UNIQUE,
    sort int  NOT NULL DEFAULT 0
);

CREATE TABLE skills (
    id      int      GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name    text     NOT NULL,
    level   smallint NOT NULL CONSTRAINT skills_level_check CHECK (level BETWEEN 1 AND 5),
    sort    int      NOT NULL DEFAULT 0,
    area_id int      NOT NULL REFERENCES skill_areas (id) ON DELETE CASCADE,
    CONSTRAINT skills_area_name_key UNIQUE (area_id, name)
);

-- The home page's slides, each with a tab.
CREATE TABLE hero_slides (
    id        int  GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    sort      int  NOT NULL DEFAULT 0,
    tab_label text NOT NULL,
    eyebrow   text NOT NULL DEFAULT '',
    title     text NOT NULL,
    subtitle  text NOT NULL DEFAULT '',
    callout   text NOT NULL DEFAULT '',
    cta_label text NOT NULL,
    cta_href  text NOT NULL,
    media_id  uuid REFERENCES media (id) ON DELETE SET NULL,
    poster_id uuid REFERENCES media (id) ON DELETE SET NULL
);

-- The contact inbox.
CREATE TABLE messages (
    id         uuid        PRIMARY KEY,
    name       text        NOT NULL,
    email      text        NOT NULL,
    body       text        NOT NULL,
    user_agent text,
    created_at timestamptz NOT NULL DEFAULT now(),
    read_at    timestamptz
);
CREATE INDEX messages_created_at_idx ON messages (created_at DESC);

-- Changes to the records, named: the table and the changed row's key (the column named
-- by the trigger's argument), so the cache can drop just what changed.
CREATE FUNCTION notify_record_changed() RETURNS trigger AS $$
DECLARE
    changed jsonb := to_jsonb(COALESCE(NEW, OLD));
BEGIN
    PERFORM pg_notify(
        'content_changed',
        json_build_object('table', TG_TABLE_NAME, 'key', changed ->> TG_ARGV[0])::text
    );
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER projects_changed AFTER INSERT OR UPDATE OR DELETE ON projects
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('slug');
CREATE TRIGGER project_tools_changed AFTER INSERT OR UPDATE OR DELETE ON project_tools
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('project_slug');
CREATE TRIGGER media_changed AFTER UPDATE OR DELETE ON media
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
CREATE TRIGGER hero_slides_changed AFTER INSERT OR UPDATE OR DELETE ON hero_slides
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
CREATE TRIGGER experience_changed AFTER INSERT OR UPDATE OR DELETE ON experience
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('slug');
CREATE TRIGGER experience_tools_changed AFTER INSERT OR UPDATE OR DELETE ON experience_tools
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('experience_id');
CREATE TRIGGER skills_changed AFTER INSERT OR UPDATE OR DELETE ON skills
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
CREATE TRIGGER skill_areas_changed AFTER INSERT OR UPDATE OR DELETE ON skill_areas
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
CREATE TRIGGER tools_changed AFTER INSERT OR UPDATE OR DELETE ON tools
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
CREATE TRIGGER tool_categories_changed AFTER INSERT OR UPDATE OR DELETE ON tool_categories
    FOR EACH ROW EXECUTE FUNCTION notify_record_changed('id');
