#!/bin/sh
# Makes the live site's data what this machine's is: Postgres, Mongo and the media volume are
# replaced by the local ones, and Redis, which holds only caches and counters, is emptied.
# The server's contact messages stay: they come from visitors, not from here.
#   scripts/push-data.sh [ssh-host]
# The host comes from the argument or DEPLOY_HOST in .env; the server's checkout from
# DEPLOY_DIR (default ~/portfolio). The local stack must be running.
set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
env_value() { sed -n "s/^$1=//p" "$root/.env" | tail -n 1; }

host=${1:-${DEPLOY_HOST:-$(env_value DEPLOY_HOST)}}
dir=${DEPLOY_DIR:-$(env_value DEPLOY_DIR)}
dir=${dir:-portfolio}
[ -n "$host" ] || { echo "usage: scripts/push-data.sh <ssh-host>, or DEPLOY_HOST in .env" >&2; exit 1; }

project=${COMPOSE_PROJECT_NAME:-$(basename "$root" | tr '[:upper:]' '[:lower:]')}
remote() { ssh "$host" "cd $dir && $1"; }

printf 'Replace the data on %s with this machine'"'"'s? [y/N] ' "$host"
read -r answer
[ "$answer" = y ] || exit 1

cd "$root"
remote 'docker compose stop backend frontend' >/dev/null 2>&1

# messages is left out whole, so the server's own rows are neither dropped nor replaced.
docker compose exec -T postgres sh -c \
    'pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" --clean --if-exists --no-owner -T messages' |
    remote 'docker compose exec -T postgres sh -c "psql -q -v ON_ERROR_STOP=1 -U \"\$POSTGRES_USER\" -d \"\$POSTGRES_DB\""' >/dev/null
echo "postgres pushed"

docker compose exec -T mongo mongodump --quiet --db portfolio --archive |
    remote 'docker compose exec -T mongo mongorestore --quiet --drop --archive'
echo "mongo pushed"

# Media comes from data/media, what `stackup dev` serves. COPYFILE_DISABLE keeps macOS tar
# from adding ._ files, which the backend would register as media.
COPYFILE_DISABLE=1 tar -C "$root/data/media" --exclude '._*' -cf - . |
    ssh "$host" "docker run --rm -i -v ${project}_media:/to alpine sh -c \
        'find /to -mindepth 1 -delete && tar -C /to -xf - && chown -R 65532:65532 /to && echo \"\$(ls /to | wc -l) media files pushed\"'"

remote 'docker compose exec -T redis redis-cli FLUSHALL && docker compose start backend frontend' >/dev/null 2>&1
echo "redis emptied, site restarted"
