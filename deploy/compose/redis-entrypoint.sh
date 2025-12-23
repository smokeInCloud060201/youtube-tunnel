#!/bin/sh
set -e

REDIS_USERNAME=${REDIS_USERNAME:-default}
REDIS_PASSWORD=${REDIS_PASSWORD:-redispassword123}

# Create redis.conf with ACL user configuration
cat > /tmp/redis.conf <<EOF
appendonly yes
user $REDIS_USERNAME on >$REDIS_PASSWORD ~* +@all
EOF

# Start Redis server with the configuration
exec redis-server /tmp/redis.conf

