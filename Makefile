chain-simulator:
	@docker compose -f docker-compose.yml build
	@docker compose -f docker-compose.yml up & sc-meta test -c
	@docker compose -f docker-compose.yml down -v