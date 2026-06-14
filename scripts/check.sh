#!/usr/bin/env bash
set -euo pipefail

python3 -m pip install --quiet jsonschema
python3 scripts/validate_dsl.py
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
