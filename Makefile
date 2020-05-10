build:
	cargo build --release

user-install: build
	mkdir -p ~/.local/bin/
	cp ./target/release/mao ~/.local/bin/
