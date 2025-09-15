# INCLUDE ==============================================================================================================
# Include Makefile
include $(ROOT_DIR)/ops/Makefile/docker.mk

### Runnings ===========================================================================================================
up: ## Run for development mode
	@COMPOSE_PROFILES=dns,observability,gateway docker compose \
		-f $(ROOT_DIR)/docker-compose.yaml \
		up -d --remove-orphans --build

down: confirm ## Down docker compose
	@COMPOSE_PROFILES=dns,observability,gateway docker compose \
		-f $(ROOT_DIR)/docker-compose.yaml \
	    down --remove-orphans
	@docker network prune -f

### Code style =========================================================================================================
lint: ## Lint code
	@cargo fmt
	@cargo clippy --fix --allow-dirty --allow-no-vcs

### Testing ============================================================================================================
test: ## Run all tests
	@cargo test

test-unit: ## Run unit tests only
	@cargo test --lib --bins

test-cucumber: ## Run cucumber tests
	@cargo test --test cucumber_simple
	@cargo test --test cucumber_expressions

test-integration: ## Run integration tests
	@cargo test --test cucumber_simple --test cucumber_expressions

test-coverage: ## Generate test coverage report
	@cargo tarpaulin --out html --output-dir ./coverage
