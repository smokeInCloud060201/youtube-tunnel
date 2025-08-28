DOCKER_BIN=docker
DOCKER_COMPOSE_BIN=docker-compose
PROJECT_NAME=youtube-tunnel
DOCKER_USER_NAME=tnkhanh

build-image:
	$(DOCKER_BIN) build -t $(DOCKER_USER_NAME)/$(PROJECT_NAME):latest .
push-image:
	$(DOCKER_BIN) push $(DOCKER_USER_NAME)/$(PROJECT_NAME)
compose:
	$(DOCKER_COMPOSE_BIN) up -d

release: build-image push-image
