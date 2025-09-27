DOCKER_BIN=docker
DOCKER_COMPOSE_BIN=docker compose
DOCKER_BASE_PATH=./deploy/docker
DOCKER_COMPOSE_BASE_PATH=./deploy/compose
DOCKER_NETWORK_NAME=yt-network


build-api-image:
	$(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/api.Dockerfile -t youtube-tunnel-api:${IMAGE_TAG} ./api

build-worker-image:
	$(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/worker.Dockerfile -t youtube-tunnel-worker:${IMAGE_TAG} ./api

build-web-image:
	$(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/web.Dockerfile -t youtube-tunnel-web:${IMAGE_TAG} ./web

deploy-common:
	$(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/base-docker-compose.yml up -d

deploy-service:
	$(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/app-docker-compose.yml up -d

create-network:
	$(DOCKER_BIN) network inspect $(DOCKER_NETWORK_NAME) >/dev/null 2>&1 || $(DOCKER_BIN) network create $(DOCKER_NETWORK_NAME)

