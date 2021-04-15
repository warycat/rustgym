#!/bin/bash
TAG=v0.2.11
VM_NAME=rustgym-24
SERVER_NAME=rustgym.com
WORK_DIR=/root
EMAIL=larry.fantasy@gmail.com
RUSTGYM_DOWNLOAD=https://github.com/warycat/rustgym/releases/download
SONIC_DOWNLOAD=https://github.com/valeriansaliou/sonic/releases/download
SONIC_TAG=v1.3.0

HTTP_UPGRADE='$http_upgrade'
HOST='$host'

IMAGE=debian-10-buster-v20201216
IMAGE_FAMILY=debian-10
MACHINE_TYPE=e2-micro
ZONE=us-central1-a
gcloud compute instances create $VM_NAME \
    --machine-type $MACHINE_TYPE \
    --zone $ZONE \
    --tags http-server,https-server \
    --metadata startup-script="#! /bin/bash
apt update
apt -y install nginx sqlite3 certbot python-certbot-nginx telnet build-essential
cd $WORK_DIR

cat <<EOF > certbot.sh
#!/bin/bash
certbot --nginx --redirect --non-interactive --agree-tos -m $EMAIL -d $SERVER_NAME
EOF
chmod u+x certbot.sh

curl -LJO $SONIC_DOWNLOAD/$SONIC_TAG/$SONIC_TAG-x86_64.tar.gz
tar -xzf $SONIC_TAG-x86_64.tar.gz
mv sonic/sonic sonic-server
mv sonic/config.cfg .
rm $SONIC_TAG-x86_64.tar.gz
rmdir sonic
mkdir -p data/store/kv
mkdir -p data/store/fst
./sonic-server >> sonic.log &>> sonic.error.log &

curl -LJO $RUSTGYM_DOWNLOAD/$TAG/pkg.tar.gz
tar -xzf pkg.tar.gz
rm pkg.tar.gz
curl -LJO $RUSTGYM_DOWNLOAD/$TAG/static.tar.gz
tar -xzf static.tar.gz
rm static.tar.gz
curl -LJO $RUSTGYM_DOWNLOAD/$TAG/rustgym.sqlite
curl -LJO $RUSTGYM_DOWNLOAD/$TAG/rustgym-server
curl -LJO $RUSTGYM_DOWNLOAD/$TAG/rustgym-ingest
chmod u+x rustgym-ingest
./rustgym-ingest >> ingest.log &>> ingest.error.log &
chmod u+x rustgym-server
TAG=$TAG ./rustgym-server >> server.log &>> server.error.log &

cat <<\EOF > /etc/nginx/sites-available/rustgym-nginx.cfg
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    server_name $SERVER_NAME;
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $HTTP_UPGRADE;
        proxy_set_header Connection \"upgrade\";
        proxy_set_header Host $HOST;
    }
}
EOF
rm /etc/nginx/sites-enabled/default
ln -s /etc/nginx/sites-available/rustgym-nginx.cfg /etc/nginx/sites-enabled/default
systemctl restart nginx
"
