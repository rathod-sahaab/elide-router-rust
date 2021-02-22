watch:
	docker-compose -f docker-compose-dev.yaml --env-file .env.dev up -d && docker-compose -f docker-compose-dev.yaml exec elide_dev bash

dev-build:
	docker-compose -f docker-compose-dev.yaml --env-file .env.dev  build

prod-build:
	docker-compose -f docker-compose.yaml --env-file .env.prod  build

stop:
	docker-compose -f docker-compose-dev.yaml --env-file .env.dev down
