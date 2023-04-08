_default:
	just --list

check:
	cargo clippy --all-features --all-targets

test *args:
	cargo nextest run {{args}}

test:
	#!/bin/bash -eux
	source <(cargo llvm-cov show-env --export-prefix)
	cargo llvm-cov clean --workspace
	cargo nextest run --profile ci
	cargo llvm-cov report --lcov --output-path coverage.lcov
