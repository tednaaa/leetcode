test:
	@cargo test -p $(lib) --lib -- --nocapture

lint:
	cargo clippy --all-targets --all-features
