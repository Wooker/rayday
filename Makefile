#!/usr/bin/make

include ./scripts/init.mf

DB_PATH = ~/.config/rayday/events.db

---------------: ## Running ---------------
r: ## Run the binary with cargo
	cargo r

b: ## Build the binary with cargo
	cargo b

---------------: ## Log commands ---------------
log: ## Cat log file
	cat log.txt

log-tr: ## Truncate the log file
	truncate -s0 log.txt


---------------: ## Database commands ---------------
db: ## Enter the database with `sqlite3`
	sqlite3 $(DB_PATH)

db-list: ## Execute database command to list all events
	sqlite3 $(DB_PATH) "select * from events;"

