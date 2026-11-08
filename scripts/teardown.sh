#!/bin/sh
# Removes the deployment from the server: containers, volumes, images, this checkout and Docker.
# k3s keeps its own containerd under /var/lib/rancher/k3s, which this leaves alone.
set -eu

# Runs from /tmp, because it deletes the folder it lives in.
if [ "$(dirname "$0")" != /tmp ]; then
	cp "$0" /tmp/portfolio-teardown.sh
	exec sh /tmp/portfolio-teardown.sh "$(cd "$(dirname "$0")/.." && pwd)"
fi

repo="$1"
cd "$repo"
docker compose down -v --rmi all
cd /
rm -rf "$repo"
sudo apt-get purge -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo rm -rf /var/lib/docker /var/lib/containerd /etc/docker
rm -f /tmp/portfolio-teardown.sh
echo "portfolio removed"
