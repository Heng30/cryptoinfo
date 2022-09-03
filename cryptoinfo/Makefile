#!/bin/bash

TARGET_DIR := ../target/release
TARGET := cryptoinfo
RELEASE_DIR := ./release
DATA_DIR := ~/.local/share/${TARGET}
CONFIG_DIR := ~/.config/${TARGET}
CUR_DIR := $(shell pwd)

build-debug:
	cargo build

build:
	cargo build --release

run:
	LD_LIBRARY_PATH=/usr/local/lib:${CUR_DIR}/../ffi/lib RUST_LOG=error,warn,info,debug,reqwest=off cargo run

install: build
	cp ${TARGET_DIR}/${TARGET} ~/.local/bin

release: make_release_dir build
	cp ${TARGET_DIR}/${TARGET} ${RELEASE_DIR}

make_release_dir:
	-mkdir ${RELEASE_DIR}

package: release
	./pack.sh

send2ubuntu:
	rsync -arv release/cryptoinfo-linux-*.run blue@192.168.0.106:

clean:
	cargo clean

dry_run:
	-cp ${DATA_DIR}/marked.dat ${RELEASE_DIR}
	-cp ${CONFIG_DIR}/translation.dat ${RELEASE_DIR}
	-rm -rf ${DATA_DIR}
	-rm -rf ${CONFIG_DIR}
	cargo run --release
