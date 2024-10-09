slow-tests:
	@docker compose -f docker-compose.yml build
	@docker compose -f docker-compose.yml up & sc-meta test-interactors
	@docker compose -f docker-compose.yml down -v