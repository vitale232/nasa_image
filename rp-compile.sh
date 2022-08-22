#/usr/bin/sh

PI_IP=192.168.1.138
TARGET=armv7-unknown-linux-gnueabihf

cargo build --target $TARGET --release

sshpass -f .pi_pass scp -r ./target/$TARGET/release/nasa_image vitale232@$PI_IP:/home/vitale232/nasa/nasa_image
sshpass -f .pi_pass scp -r ./.env vitale232@$PI_IP:/home/vitale232/nasa/.env

