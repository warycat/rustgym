#!/bin/bash
TAG=v0.1.9
VM_NAME=rustgym-29
SERVER_NAME=rustgym.com
WORK_DIR=/root
EMAIL=larry.fantasy@gmail.com

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
apt -y install nginx sqlite3 certbot python-certbot-nginx
cat <<EOF > /etc/nginx/sites-available/rustgym-nginx.cfg
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    server_name $SERVER_NAME;
    location / {
        proxy_pass http://127.0.0.1:8080;
    }
}
EOF
rm /etc/nginx/sites-enabled/default
ln -s /etc/nginx/sites-available/rustgym-nginx.cfg /etc/nginx/sites-enabled/default
cd $WORK_DIR
cat <<EOF > certbot.sh
#!/bin/bash
certbot --nginx --redirect --non-interactive --agree-tos -m $EMAIL -d $SERVER_NAME
EOF
chmod u+x certbot.sh
curl -LJO https://github.com/warycat/rustgym/releases/download/$TAG/rustgym.sqlite --output rustgym.sqlite
curl -LJO https://github.com/warycat/rustgym/releases/download/$TAG/rustgym-server --output rustgym-server
chmod u+x rustgym-server
TAG=$TAG ./rustgym-server >> rustgym.log &>> rustgym.error.log &
systemctl restart nginx
"
