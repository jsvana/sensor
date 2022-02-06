BINARY="sensor"
TARGET="armv7-unknown-linux-gnueabihf"
HOST="192.168.1.140"
BROKER_HOST="192.168.1.133"

deploy:
	cross build --target ${TARGET} --release && scp target/${TARGET}/release/${BINARY} ${HOST}:${BINARY}

run: deploy
	ssh ${HOST} './${BINARY} --broker-address tcp://${BROKER_HOST}:1883 --room office'
