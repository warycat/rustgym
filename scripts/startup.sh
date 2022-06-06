#!/bin/bash
source scripts/ignore.env
source scripts/const.env

apt update
apt install -y screen git telnet certbot coturn debian-keyring debian-archive-keyring apt-transport-https curl lsb-release
if [ ! -f /usr/share/keyrings/getenvoy-keyring.gpg ]
then
    curl -sL 'https://deb.dl.getenvoy.io/public/gpg.8115BA8E629CC074.key' | sudo gpg --dearmor -o /usr/share/keyrings/getenvoy-keyring.gpg
fi
echo a077cb587a1b622e03aa4bf2f3689de14658a9497a9af2c427bba5f4cc3c4723 /usr/share/keyrings/getenvoy-keyring.gpg | sha256sum --check
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/getenvoy-keyring.gpg] https://deb.dl.getenvoy.io/public/deb/debian $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/getenvoy.list
apt update
apt install -y getenvoy-envoy

curl -LJO $SONIC_DOWNLOAD/$SONIC_TAG/$SONIC_TAG-x86_64.tar.gz
tar -xzf $SONIC_TAG-x86_64.tar.gz
mv sonic/sonic sonic-server
rm $SONIC_TAG-x86_64.tar.gz
rmdir sonic

mkdir -p data/store/kv
mkdir -p data/store/fst
mkdir -p log
mv ./target/x86_64-unknown-linux-musl/release/* .

git clone https://github.com/ua-parser/uap-core.git

source scripts/sonic.sh
source scripts/ingest.sh
source scripts/coturn.sh
source scripts/server.sh
