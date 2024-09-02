new-solution: check-lib
	@cargo init --lib crates/$(lib)

test: check-lib
	@cargo test -p $(lib) --lib -- --nocapture

lint:
	@cargo clippy --all-targets --all-features


check-lib:
	@if [ -z "$(lib)" ]; then \
		echo "Error: 'lib' argument is required."; \
		exit 1; \
	fi

