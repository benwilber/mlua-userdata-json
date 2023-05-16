.PHONY: all build release clean format lint doc test ready

all: build

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

format:
	cargo fmt

lint:
	cargo fmt --check
	cargo check

doc:
	cargo doc

test:
	cargo test

ready: format lint test doc
	@echo "Ready!"
