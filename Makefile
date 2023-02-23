all:
	cargo build --release
	cp target/release/program-rust randmst
