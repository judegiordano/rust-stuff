.PHONY: kill build up restart

include .env
export
DOCKER_PROJECT = rust_by_example

kill:
	docker compose kill

build:
	docker compose -f docker-compose.yml -d build

up:
	docker compose -f docker-compose.yml -d --up

restart:
    docker-compose -f docker-compose.yml stop
    docker-compose -f docker-compose.yml up -d
