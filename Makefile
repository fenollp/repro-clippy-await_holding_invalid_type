all:
	cargo clippy --locked --frozen --offline --all-targets --all-features -- --no-deps -A clippy::all -W clippy::await_holding_invalid_type
