.PHONY: up down build clean ps

ENV_FILE := .devcontainer/.env

up:
	@sed -i '/^HOST_UID=/d;/^HOST_GID=/d' $(ENV_FILE)
	@echo "HOST_UID=$$(id -u)" >> $(ENV_FILE)
	@echo "HOST_GID=$$(id -g)" >> $(ENV_FILE)
	@awk '!seen[$$0]++' $(ENV_FILE) > $(ENV_FILE).tmp && mv $(ENV_FILE).tmp $(ENV_FILE)
	docker compose -f .devcontainer/docker-compose.yml --env-file $(ENV_FILE) up -d

down:
	docker compose -f .devcontainer/docker-compose.yml --env-file $(ENV_FILE) down

build:
	docker compose -f .devcontainer/docker-compose.yml --env-file $(ENV_FILE) build

clean: down
	sudo rm -rf .data/postgres/*

ps:
	docker compose -f .devcontainer/docker-compose.yml ps
