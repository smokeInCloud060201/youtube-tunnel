DOCKER_BIN=docker
DOCKER_COMPOSE_BIN=docker compose
DOCKER_BASE_PATH=./deploy/docker
DOCKER_COMPOSE_BASE_PATH=./deploy/compose
DOCKER_NETWORK_NAME=yt-network
IMAGE_TAG=latest


.PHONY: clean-api clean-worker clean-backend

build-api-image:
	$(DOCKER_BIN) rmi -f youtube-tunnel-api:${IMAGE_TAG} || true
	DOCKER_BUILDKIT=1 $(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/api.Dockerfile -t youtube-tunnel-api:${IMAGE_TAG} ./backend/api

build-worker-image:
	$(DOCKER_BIN) rmi -f youtube-tunnel-worker:${IMAGE_TAG} || true
	DOCKER_BUILDKIT=1 $(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/worker.Dockerfile -t youtube-tunnel-worker:${IMAGE_TAG} ./backend/worker

build-web-image:
	$(DOCKER_BIN) rmi -f youtube-tunnel-web:${IMAGE_TAG} || true
	$(DOCKER_BIN) build -f $(DOCKER_BASE_PATH)/web.Dockerfile -t youtube-tunnel-web:${IMAGE_TAG} ./web

deploy-service:
	 $(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/app-docker-compose.yml up -d --build

create-network:
	 $(DOCKER_BIN) network inspect $(DOCKER_NETWORK_NAME) >/dev/null 2>&1 || $(DOCKER_BIN) network create $(DOCKER_NETWORK_NAME)

service-down:
	 $(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/app-docker-compose.yml down || true

deploy-service-local:
	 $(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/app-docker-compose.local.yml up -d

deploy-common-local:
	 $(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/base-docker-compose.local.yml up -d


deploy-local: deploy-service-local

clean-app-images:
	$(DOCKER_COMPOSE_BIN) -f $(DOCKER_COMPOSE_BASE_PATH)/app-docker-compose.yml down --rmi all -v --remove-orphans || true
	$(DOCKER_BIN) rmi -f youtube-tunnel-api:latest youtube-tunnel-worker:latest youtube-tunnel-web:latest || true


deploy: create-network service-down deploy-service

