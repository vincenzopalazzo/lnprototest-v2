CC=cargo
FMT=fmt

ARGS="--all"

default: fmt
	$(CC) build

fmt:
	$(CC) fmt --all

check: ## Runs unit testing
	$(CC) build --all
	$(CC) test $(ARGS) -- --nocapture

clean: ## Clean up everythings
	$(CC) clean

help: ## Show Help
	@grep --no-filename -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-15s\033[0m %s\n", $$1, $$2}'