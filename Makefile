slow-tests:
	@docker compose -f docker/docker-compose.yml build
	@docker compose -f docker/docker-compose.yml up & cargo test --features chain_simulator
	@docker compose -f docker/docker-compose.yml down -v