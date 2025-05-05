#!/bin/bash

set -e

BIN="$(cd "$(dirname "$0")" ; pwd)"
PROJECT="$(dirname "${BIN}")"

source "${BIN}/lib-verbose.sh"

(
  cd "${PROJECT}"
  export RUST_LOG='info,thread_per_core=debug'
  cargo run -- "$@"
)
