# Setup
.PHONY: all run install
.DEFAULT_GOAL := run  # Make it default since we have nothing else now

# Variables
PY3_PROG := python3.10

# Targets
all:  # Not in use
	@echo -e "Invoke \`run\` target to run the test.\nHalting..."

run:
	@echo -n "Using " && echo ${PY3_PROG}
	@${PY3_PROG} test.py
