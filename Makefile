#!/bin/bash

TARGET_DIR := ./target/release
TARGET := cryptoinfo
RELEASE_DIR := ./release
DATA_DIR := ~/.local/share/${TARGET}
CONFIG_DIR := ~/.config/${TARGET}

build-debug:
	cargo build

build:
	cargo build --release

run:
	RUST_LOG=error,warn,info,debug,trace cargo run --release

install: release
	cp ${RELEASE_DIR}/${TARGET} ~/.local/bin
	-cp ${RELEASE_DIR}/marked.dat ${DATA_DIR}
	-cp ${RELEASE_DIR}/translation.dat ${CONFIG_DIR}

release: make_release_dir build
	cp ${TARGET_DIR}/${TARGET} ${RELEASE_DIR}

make_release_dir:
	-mkdir ${RELEASE_DIR}

clean:
	cargo clean

dry_run:
	-cp ${DATA_DIR}/marked.dat ${RELEASE_DIR}
	-cp ${CONFIG_DIR}/translation.dat ${RELEASE_DIR}
	-rm -rf ${DATA_DIR}
	-rm -rf ${CONFIG_DIR}
	cargo run --release
