#!/bin/sh
# Uploads a photo or video and prints its file name (`<sha256>.<ext>`): what the seeds
# use to name media, as a project's `logo`, a slide's `media` or a block's `file`.
#   scripts/upload-media.sh hero.mp4 [api-url]
# The token comes from ADMIN_TOKEN in .env; the API URL from the argument or API_URL in .env.
set -eu

file=${1:?usage: scripts/upload-media.sh <file> [api-url]}
root=$(cd "$(dirname "$0")/.." && pwd)
env_value() { sed -n "s/^$1=//p" "$root/.env" | tail -n 1; }

token=${ADMIN_TOKEN:-$(env_value ADMIN_TOKEN)}
api=${2:-${API_URL:-$(env_value API_URL)}}

case $(printf '%s' "$file" | tr '[:upper:]' '[:lower:]') in
    *.mp4) type=video/mp4 ;;
    *.webm) type=video/webm ;;
    *.jpg | *.jpeg) type=image/jpeg ;;
    *.png) type=image/png ;;
    *.webp) type=image/webp ;;
    *.avif) type=image/avif ;;
    *) echo "unsupported file type: $file" >&2; exit 1 ;;
esac

# -k: the local certificate is self-signed. The answer is {"id": …, "url": "/media/<name>"}.
curl --fail-with-body -sS -k -X POST \
    -H "Authorization: Bearer $token" \
    -H "Content-Type: $type" \
    --data-binary "@$file" \
    "$api/media" |
    sed -n 's|.*"url":"/media/\([^"]*\)".*|\1|p'
