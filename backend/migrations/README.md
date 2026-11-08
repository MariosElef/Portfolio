# Migrations

Each file here is applied once, in order, when the backend starts, and is compiled into it.
An applied migration must never change: sqlx checks each one's checksum. Add the next one
with `sqlx migrate add -r <name>`.

**Migrations hold schema only.** Tables, columns, constraints, indexes and triggers go here.
Content does not: the records the site starts with live in `backend/src/seeds/records/`,
and the documents in `backend/src/seeds/`. Start-up writes them wherever they are missing.
`ci.sh` rejects any migration with `INSERT`, `UPDATE` or `DELETE` in it.

The one exception is moving existing rows into a new shape when the schema changes. Even
then, the migration only carries rows across; it never decides what they are.

Media is always named by its file (`<sha256>.<ext>`), never by id, so links rebuild on any
database that has the files. `scripts/upload-media.sh` prints the name to use.
