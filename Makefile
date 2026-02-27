TARGET := vellum

.PHONY: all build clean debug tui gui test fmt fmtc lint

all: clean fmt lint build



build:
	cargo build --release

clean:
	cargo clean

debug:
	cargo build

tui:
	cargo run --release -p vellum-tui

gui:
	cargo run --release -p vellum-gui

test:
	cargo test --release

fmt:
	cargo fmt

fmtc:
	cargo fmt --check

lint:
	cargo clippy --release -- -D warnings
