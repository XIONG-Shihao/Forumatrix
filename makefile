# Project variables
PROJECT_NAME=arena-ai
COMPOSE=docker compose
COMPOSE_FILE=docker-compose.yml

# Default target
.DEFAULT_GOAL := help

## ---- General ----
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build all services
	$(COMPOSE) -f $(COMPOSE_FILE) build

up: ## Start all services in background
	$(COMPOSE) -f $(COMPOSE_FILE) up -d

down: ## Stop all services
	$(COMPOSE) -f $(COMPOSE_FILE) down

logs: ## Tail logs for all services
	$(COMPOSE) -f $(COMPOSE_FILE) logs -f

restart: down up ## Restart all services

## ---- Backend ----
backend-build: ## Rebuild backend only
	$(COMPOSE) -f $(COMPOSE_FILE) build api

backend-shell: ## Open shell inside backend container
	$(COMPOSE) -f $(COMPOSE_FILE) exec api sh

backend-logs: ## Show backend logs
	$(COMPOSE) -f $(COMPOSE_FILE) logs -f api

backend-migrate: ## Run database migrations
	$(COMPOSE) -f $(COMPOSE_FILE) run --rm api sqlx migrate run

## ---- Frontend ----
frontend-build: ## Rebuild frontend only
	$(COMPOSE) -f $(COMPOSE_FILE) build web

frontend-shell: ## Open shell inside frontend container
	$(COMPOSE) -f $(COMPOSE_FILE) exec web sh

frontend-logs: ## Show frontend logs
	$(COMPOSE) -f $(COMPOSE_FILE) logs -f web

## ---- Utilities ----
clean: ## Stop and remove containers + volumes
	$(COMPOSE) -f $(COMPOSE_FILE) down -v

ps: ## Show running containers
	$(COMPOSE) -f $(COMPOSE_FILE) ps

# ---- Convenience builds/restarts ----
rebuild:
	$(COMPOSE) -f $(COMPOSE_FILE) up -d --no-deps --build api
	$(COMPOSE) -f $(COMPOSE_FILE) down
