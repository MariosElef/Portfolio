#!/bin/sh
set -eu

# One command per line: under `set -e`, a failure inside an `a && b` chain does not stop
# the script, so a chain would let a failing check pass.
# Compile-time query checking against the committed backend/.sqlx, as the Docker build does.
export SQLX_OFFLINE=true

cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
# SQL: one query per file, upper-case keywords, columns named by their table (backend/.sqruff).
(cd backend && sqruff lint src/repositories/postgres/sql)

# Layers: each may use only the ones below it.
#
# backend:  routes → services → repositories / clients → models
#           middleware and jobs beside them; seeds are data built from models;
#           config and startup wire it all together before serving
# frontend: app → features → components / composables → services → types / utils
#           middleware beside them

# backend: only repositories and clients talk to the databases and Redis.
# (The drivers' own paths, not this crate's `repositories::redis`.)
if grep -rnE '(^|[^:[:alnum:]_])(sqlx|redis|mongodb)::' backend/src/routes backend/src/services \
    | grep -vE 'redis::(keys|Redis|cache|rate_limit)\b'; then exit 1; fi
# backend: storage is split by database, each directory using only its own driver.
if grep -rnE 'mongodb::|redis::' backend/src/repositories/postgres*; then exit 1; fi
if grep -rnE 'sqlx::|redis::' backend/src/repositories/mongo*; then exit 1; fi
if grep -rnE 'sqlx::|mongodb::' backend/src/repositories/redis*; then exit 1; fi
# backend: routes go through services, never straight to storage.
if grep -rnE 'crate::(clients|repositories)::' backend/src/routes; then exit 1; fi
# backend: nothing below routes reaches up.
if grep -rnE 'crate::routes' backend/src/services* backend/src/repositories* backend/src/clients* backend/src/jobs*; then exit 1; fi
if grep -rnE 'crate::services' backend/src/repositories* backend/src/clients*; then exit 1; fi
# backend: only start-up writes the starting content; storage never decides it.
if grep -rnE 'crate::seeds' backend/src/routes* backend/src/services* backend/src/repositories* backend/src/clients* backend/src/jobs*; then exit 1; fi
# backend: seeds are data, built from the models alone.
if grep -rnE 'crate::(clients|jobs|repositories|routes|services)' backend/src/seeds*; then exit 1; fi
# backend: migrations hold schema only; content starts from the seeds.
if grep -lniE '^[[:space:]]*(INSERT|UPDATE|DELETE)[[:space:]]' backend/migrations/*.up.sql; then
    echo "content belongs in backend/src/seeds" >&2
    exit 1
fi

# frontend: Tailwind reads class names as they are written, so a variant spliced in by
# format!, as "{HELD}:bg-white", is never generated. Write variants out in full.
if grep -rnE '\{[A-Za-z_]+\}:[a-z!\[]' frontend/src; then exit 1; fi
# frontend: components render what they are given; they fetch nothing and know no routes.
if grep -rnEs 'crate::(app|features|composables|services|middleware)' frontend/src/components*; then exit 1; fi
# frontend: composables load data; they draw nothing.
if grep -rnEs 'crate::(app|features|components|middleware)' frontend/src/composables*; then exit 1; fi
# frontend: the API client knows only the data's shape.
if grep -rnEs 'crate::(app|features|components|composables|middleware)' frontend/src/services*; then exit 1; fi
# frontend: features get their data from composables, and their links from the pages.
if grep -rnEs 'crate::(app|services)' frontend/src/features*; then exit 1; fi
# frontend: the bottom layers depend on nothing above them.
if grep -rnEs 'crate::(app|features|components|composables|services)' frontend/src/middleware* frontend/src/types* frontend/src/utils*; then exit 1; fi
# frontend: pages and features are made of components only; any HTML is drawn by a
# component. (A `<` after a letter or `::` is a generic, such as `Result<impl View>`.)
if grep -rnE '(^|[^A-Za-z0-9_:])</?[a-z]' frontend/src/app frontend/src/app.rs frontend/src/features | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'; then exit 1; fi
