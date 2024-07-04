## update and install packages necessary for the environment
apt-get update
apt-get install -y \
  curl \
  git \
  jq \
  sudo \
  vim \
  build-essential \
  openssl

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install nightly