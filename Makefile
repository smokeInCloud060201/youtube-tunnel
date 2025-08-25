DOCKER_BIN=docker
DOCKER_COMPOSE_BIN=docker-compose

setup:
	$(DOCKER_BIN) build -t api:latest .
compose:
	$(DOCKER_COMPOSE_BIN) up -d