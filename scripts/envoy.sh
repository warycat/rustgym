#!/bin/bash
envoy -c config/envoy.yaml 2>&1 >> log/envoy.log &