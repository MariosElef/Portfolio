#!/bin/sh
# Copies the media in data/media into the Docker `media` volume, which the backend and
# Caddy use when they run in Docker. Files are named by their hash, so this only adds
# what is missing and never overwrites. The backend registers new files on its next start.
#   scripts/import-media.sh
set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
project=${COMPOSE_PROJECT_NAME:-$(basename "$root" | tr '[:upper:]' '[:lower:]')}
volume="${project}_media"

docker volume inspect "$volume" >/dev/null 2>&1 || docker volume create "$volume" >/dev/null
docker run --rm \
    -v "$root/data/media:/from:ro" \
    -v "$volume:/to" \
    alpine sh -c 'cp -n /from/* /to/ 2>/dev/null; echo "$(ls /to | wc -l) files in the volume"'
