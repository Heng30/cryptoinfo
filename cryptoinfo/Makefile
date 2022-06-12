#!/bin/bash

TARGET_DIR := ../target/release
TARGET := cryptoinfo
RELEASE_DIR := ./release
DATA_DIR := ~/.local/share/${TARGET}
CONFIG_DIR := ~/.config/${TARGET}

build-debug:
	cargo build

build:
	cargo build --release

run:
	ROCKET_PORT=8080 ROCKET_ENV=development RUST_LOG=error,warn,info,debug,reqwest=off,rocket=off cargo run

install: release
	cp ${RELEASE_DIR}/${TARGET} ~/.local/bin
	-cp ${RELEASE_DIR}/translation.dat ${CONFIG_DIR}

release: make_release_dir build
	cp ${TARGET_DIR}/${TARGET} ${RELEASE_DIR}

make_release_dir:
	-mkdir ${RELEASE_DIR}

package: release
	./make_package.sh

send2ubuntu:
	rsync -arv release/package blue@192.168.0.106:cryptoinfo

clean:
	cargo clean

dry_run:
	-cp ${DATA_DIR}/marked.dat ${RELEASE_DIR}
	-cp ${CONFIG_DIR}/translation.dat ${RELEASE_DIR}
	-rm -rf ${DATA_DIR}
	-rm -rf ${CONFIG_DIR}
	cargo run --release
