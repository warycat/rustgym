#!/bin/bash
echo "starting sonic server"
./sonic-server -c config/sonic.cfg 2>&1 >> log/sonic.log &