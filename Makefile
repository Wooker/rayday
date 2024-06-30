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

db-drop:
	sqlite3 $(DB_PATH) "delete from events;"

db-add-test: ## Add test case for debugging
	$(eval DATE := $(shell date +%Y-%m-%d))
	sqlite3 $(DB_PATH) \
	"insert into events (description, start, end) values \
	  ('task1', '$(DATE) 12:00:00', '$(DATE) 13:00:00') \
	, ('task2', '$(DATE) 12:30:00', '$(DATE) 13:30:00') \
	, ('task2', '$(DATE) 12:10:00', '$(DATE) 13:10:00') \
	, ('task2', '$(DATE) 13:05:00', '$(DATE) 14:10:00') \
	;"

db-clear-today: ## Add test case for debugging
	$(eval DATE := $(shell date +%Y-%m-%d))
	$(eval TOMORROW := $(shell date +%Y-%m-%d -d "$(date +%Y-%m-%d) + 1 days"))
	sqlite3 $(DB_PATH) \
	"delete from events where start > '$(DATE)' and end < '$(TOMORROW)'"
