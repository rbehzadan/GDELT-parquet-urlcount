APP_NAME := $(shell sed -n 's/^name *= *"\(.*\)"/\1/p' Cargo.toml | head -1)
VERSION  ?= $(shell sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)
TAG := v$(VERSION)

TARGETS = x86_64 x86_64-musl aarch64 aarch64-musl

.PHONY: all build tag clean $(TARGETS)

build:
	@echo "Building default target..."
	cargo build --release

all: $(TARGETS)
	@echo ""
	@echo "✓ All builds completed successfully!"

x86_64:
	@echo "Building for x86_64-unknown-linux-gnu..."
	@cross build --release --target x86_64-unknown-linux-gnu
	@echo "✓ Done: x86_64-unknown-linux-gnu"

x86_64-musl:
	@echo "Building for x86_64-unknown-linux-musl..."
	@cross build --release --target x86_64-unknown-linux-musl
	@echo "✓ Done: x86_64-unknown-linux-musl"

aarch64:
	@echo "Building for aarch64-unknown-linux-gnu..."
	@cross build --release --target aarch64-unknown-linux-gnu
	@echo "✓ Done: aarch64-unknown-linux-gnu"

aarch64-musl:
	@echo "Building for aarch64-unknown-linux-musl..."
	@cross build --release --target aarch64-unknown-linux-musl
	@echo "✓ Done: aarch64-unknown-linux-musl"

tag:
	@echo "Tagging version $(TAG)..."
	@git tag $(TAG)
	@git push origin $(TAG)

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

