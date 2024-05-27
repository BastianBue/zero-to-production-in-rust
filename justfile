watch:
  cargo watch -x check -x test -x run
spin-up-db:
  ./scripts/init_db.sh
migrate-locally:
  SKIP_DOCKER=TRUE ./scripts/init_db.sh
show-udeps:
  cargo +nightly udeps
check-deps:
  cargo deny check advisories
build-docker:
  docker build -t newsletter .
run-docker:
    docker run -p 8080:8080 newsletter