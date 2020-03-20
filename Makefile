lint:
	docker-compose run web npm run lint

lint-fix:
	docker-compose run web npm run lint --fix
