watch:
	docker-compose -f docker-compose-test.yaml up -d && docker-compose -f docker-compose-test.yaml exec elide_test bash

build:
	docker-compose -f docker-compose-test.yaml build

stop:
	docker-compose -f docker-compose-test.yaml down
