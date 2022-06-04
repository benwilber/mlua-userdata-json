.PHONY: all build format lint doc ready

all: build

build:
	cargo build

format:
	cargo fmt

lint:
	cargo fmt -- --check
	cargo clippy

doc:
	cargo doc

ready: format lint
	@echo "Ready!"
