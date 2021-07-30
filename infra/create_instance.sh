#!/bin/bash
source infra/ignore.env
source infra/const.env

echo $VM_NAME $TAG

#escape keys
HTTP_UPGRADE='$http_upgrade'
HOST='$host'

gcloud compute instances create $VM_NAME \
    --machine-type $MACHINE_TYPE \
    --zone $ZONE \
    --tags http-server,https-server,stun-server \
    --metadata startup-script="#!/bin/bash
apt update
apt -y install nginx sqlite3 certbot python-certbot-nginx telnet build-essential git coturn
cd $WORK_DIR

cat <<EOF > certbot.sh
#!/bin/bash
certbot --nginx --redirect --non-interactive --agree-tos -m $EMAIL -d $SERVER_NAME
EOF
chmod u+x certbot.sh

cat <<EOF > coturn.sh
#!/bin/bash
turnserver -n --verbose --use-auth-secret --static-auth-secret=$TURN_STATIC_AUTH_SECRET --fingerprint --realm=$TURN_REALM --external-ip=$TURN_EXTERNAL_IP --cert=$TURN_CERT --pkey=$TURN_PKEY --cli-password=$TURN_CLI_PASSWORD 1> turnserver.log 2> turnserver.error.log &
EOF
chmod u+x coturn.sh

curl -LJO $SONIC_DOWNLOAD/$SONIC_TAG/$SONIC_TAG-x86_64.tar.gz
tar -xzf $SONIC_TAG-x86_64.tar.gz
mv sonic/sonic sonic-server
mv sonic/config.cfg .
rm $SONIC_TAG-x86_64.tar.gz
rmdir sonic
mkdir -p data/store/kv
mkdir -p data/store/fst
./sonic-server 1> sonic.log 2> sonic.error.log &

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
./rustgym-ingest 1> ingest.log 2> ingest.error.log &
chmod u+x rustgym-server
git clone https://github.com/ua-parser/uap-core.git
TAG=$TAG TURN_STATIC_AUTH_SECRET=$TURN_STATIC_AUTH_SECRET ./rustgym-server 1> server.log 2> server.error.log &

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
echo FINISHED
"
