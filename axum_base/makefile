run: 
	cargo run --bin server
 
run_client:
	cargo run --bin client

w_run:
	cargo watch -q -c -w src/ -x "run --bin server"

w_test:
	cargo watch -q -c -w tests/ -x "test client -- --nocapture"

docker_build:
	docker build --tag axum_base .

docker_run:
	docker run --publish 18080:18080 axum_base

