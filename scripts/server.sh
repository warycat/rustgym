#!/bin/bash
echo "starting rustgym server"
TAG=$TAG TURN_STATIC_AUTH_SECRET=$TURN_STATIC_AUTH_SECRET RUST_LOG=INFO ./rustgym-server 2>&1 >> log/server.log &
