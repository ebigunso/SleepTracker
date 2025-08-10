#!/bin/sh
set -e

# create the data directory if it doesn't exist
mkdir -p /data

# make sure our named volume is writable by UID 1000
chown -R 1000:1000 /data

# exec the real server as the unprivileged user
exec gosu 1000:1000 /app/sleep-api
