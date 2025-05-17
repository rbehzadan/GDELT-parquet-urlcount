APP_NAME := $(shell grep '^name' Cargo.toml | head -1 | cut -d '"' -f2)
VERSION ?= $(shell grep '^version' Cargo.toml | head -1 | cut -d '"' -f2)
TAG := v$(VERSION)

.PHONY: build tag release clean

build:
	cargo build --release

tag:
	git tag $(TAG)
	git push origin $(TAG)

clean:
	cargo clean

