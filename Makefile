# Cargo commands
CARGO = cargo
CARGO_BUILD = $(CARGO) build
CARGO_RUN = $(CARGO) run
CARGO_TEST = $(CARGO) test

.PHONY: run

run:
	$(CARGO_BUILD) && $(CARGO_RUN)
