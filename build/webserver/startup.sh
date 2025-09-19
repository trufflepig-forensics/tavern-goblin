#!/usr/bin/env bash

set -e

# Update ca certificates
sudo /usr/sbin/update-ca-certificates

/bin/server migrate /migrations
exec /bin/server start