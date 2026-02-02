.PHONY: build release install uninstall clean

PREFIX ?= /usr/local

build:
	cargo build

release:
	cargo build --release

install: release
	install -d $(PREFIX)/bin
	install -m 755 target/release/qp $(PREFIX)/bin/qp

uninstall:
	rm -f $(PREFIX)/bin/qp

clean:
	cargo clean
