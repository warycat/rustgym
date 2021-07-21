#!/bin/bash
source infra/const.env

gcloud compute firewall-rules create allow-stun \
    --allow tcp:$TURN_LISTENING_PORT,udp:$TURN_LISTENING_PORT,tcp:$TURN_TLS_LISTENING_PORT,udp:$TURN_TLS_LISTENING_PORT,tcp:$TURN_MIN_PORT-$TURN_MAX_PORT,udp:$TURN_MIN_PORT-$TURN_MAX_PORT \
    --target-tags stun-server \
