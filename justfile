dev:
	cargo watch -x run
fmt:
	cargo fmt
test:
	cargo test
build-image:
	docker build -t sleep-api:dev .
