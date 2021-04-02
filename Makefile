ALL_SERVICES := rust

COMPOSE_ALL_FILES := ${CORE_SERVICES_FILES}
CORE_SERVICES_FILES := -f docker-compose.yml
SERVICE=rust

#--------------------------

all: down core

build:
	@docker-compose ${COMPOSE_ALL_FILES} up -d --build ${ALL_SERVICES}

down:
	@docker-compose ${COMPOSE_ALL_FILES} down

stop:
	@docker-compose ${COMPOSE_ALL_FILES} stop ${ALL_SERVICES}

restart:
	@docker-compose ${COMPOSE_ALL_FILES} restart ${ALL_SERVICES}

rm:
	@docker-compose $(COMPOSE_ALL_FILES) rm -f ${ALL_SERVICES}

logs:
	@docker-compose $(COMPOSE_ALL_FILES) logs --follow --tail=1000 ${ALL_SERVICES}

images:
	@docker-compose $(COMPOSE_ALL_FILES) images ${ALL_SERVICES}

clean: ## Remove all Containers and Delete Volume Data
	@docker-compose ${COMPOSE_ALL_FILES} down -v

cli: ## Remove all Containers and Delete Volume Data
	@docker-compose exec ${SERVICE} sh