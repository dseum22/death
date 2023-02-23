program :=randmst

all:
	cargo build --release
	cp target/release/randmst randmst
