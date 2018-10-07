#!/bin/bash

# Exit if any subcommand fails
set -e

if [ -n "$WITH_LIBSNARK" ]; then
	cargo build
else
	cargo -Z package-features build --no-default-features
fi
