#! /bin/bash

sudo apt update && sudo apt update -y
sudo apt install docker docker-compose nginx make -y

# firewall
sudo ufw enable
sudo ufw status
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https

# create env vars file
cp .env.dev .env.prod

sudo apt-get install python3-certbot-nginx

