#!/bin/bash
turnserver -n --verbose --use-auth-secret --fingerprint \
    --static-auth-secret=$TURN_STATIC_AUTH_SECRET \
    --realm=$TURN_REALM \
    --external-ip=$TURN_EXTERNAL_IP \
    --cert=$TURN_CERT \
    --pkey=$TURN_PKEY \
    --cli-password=$TURN_CLI_PASSWORD \
    2>&1 >> log/turnserver.log &