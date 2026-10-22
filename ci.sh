#!/bin/sh
set -eu

cargo fmt --all --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked
if grep -rnE 'sqlx::|redis::|mongodb::' backend/src/routes backend/src/services; then exit 1; fi
if grep -rn 'crate::clients::' backend/src/routes; then exit 1; fi
if grep -rnEs 'crate::(app|features|services|middleware)' frontend/src/components*; then exit 1; fi
if grep -rnEs 'crate::(app|features|components|middleware)' frontend/src/composables*; then exit 1; fi
if grep -rnEs 'crate::(app|features|components|composables|middleware)' frontend/src/services*; then exit 1; fi
if grep -rnEs 'crate::app' frontend/src/features*; then exit 1; fi
if grep -rnEs 'crate::(app|features|components|composables|services)' frontend/src/middleware* frontend/src/types* frontend/src/utils*; then exit 1; fi
