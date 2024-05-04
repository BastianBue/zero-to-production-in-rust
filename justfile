watch:
  cargo watch -x check -x test -x run
spin-up-db:
  ./scripts/init_db.sh
migrate-locally:
  SKIP_DOCKER=TRUE ./scripts/init_db.sh
