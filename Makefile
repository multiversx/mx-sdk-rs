slow-tests:
	@docker compose -f docker-compose.yml build
	@docker compose -f docker-compose.yml up & cargo test --features chain_simulator --quiet
	@docker compose -f docker-compose.yml down -v