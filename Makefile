BINARY="sensor"
TARGET="armv7-unknown-linux-gnueabihf"
HOST="192.168.1.140"

deploy:
	cargo build --target ${TARGET} --release && scp target/${TARGET}/release/${BINARY} ${HOST}:${BINARY}

run: deploy
	ssh ${HOST} ./${BINARY}
