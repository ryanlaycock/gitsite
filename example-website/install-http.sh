#!/bin/sh
echo "Installing HTTP (none TLS) GitSite Example Website https://github.com/ryanlaycock/gitsite/tree/main/example-website"

# Get GITHUB_ACCESS_TOKEN
GITHUB_ACCESS_TOKEN=$1

# Install docker
# https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04

echo "Installing Docker"
sudo apt update
sudo apt install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu focal stable"
apt-cache policy docker-ce
sudo apt install docker-ce
docker --version

# Install docker-compose
# https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-compose-on-ubuntu-20-04

echo "Installing docker-compose"
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
docker-compose --version

# Download example-website docker-compose.yaml

echo "Fetching example-website docker-compose.yaml"
curl -L https://raw.githubusercontent.com/ryanlaycock/gitsite/master/example-website/docker-compose.yaml -o ryanlaycock/gitsite/example-website/

# Download nginx conf

echo "Fetching example-website nginx.conf"
curl -L https://raw.githubusercontent.com/ryanlaycock/gitsite/master/example-website/app-http.conf -o /etc/nginx/conf.d/app.conf

# docker-compose up -d example-website

echo "Running example-website docker containers"
sudo docker-compose up -d -e GITHUB_ACCESS_TOKEN=$GITHUB_ACCESS_TOKEN
