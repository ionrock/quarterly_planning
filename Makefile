.PHONY: build release install uninstall clean

PREFIX ?= /usr/local

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall qp

clean:
	cargo clean
