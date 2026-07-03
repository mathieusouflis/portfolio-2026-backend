TF_DIR := terraform/docker
DATABASE_URL := $(shell grep -E '^DATABASE_URL=' .env | cut -d '=' -f2-)

.PHONY: db-up db-down db-plan db-logs db-psql migrate

db-up:
	cd $(TF_DIR) && terraform apply -auto-approve

db-down:
	cd $(TF_DIR) && terraform destroy -auto-approve

db-plan:
	cd $(TF_DIR) && terraform plan

db-logs:
	docker logs -f portfolio-postgres

db-psql:
	docker exec -it portfolio-postgres psql -U root -d backend

migrate:
	DATABASE_URL=$(DATABASE_URL) sqlx migrate run --source crates/infrastructure/migrations
