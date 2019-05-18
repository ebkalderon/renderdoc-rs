#!/usr/bin/env bash

# This script generates Rust bindings to the in-application Renderdoc API.
#
# Dependencies:
# * bindgen
# * curl

set -euo pipefail

readonly VERSION=v1.x
readonly BASE_DIR="$(cd "$(dirname "$0")" && pwd)"
readonly TEMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/renderdoc-rs.XXXXXXXXX")"

trap -- "rm -rf '${TEMP_DIR}'" EXIT

cd "${TEMP_DIR}" || exit 1

curl -LO "https://raw.githubusercontent.com/baldurk/renderdoc/${VERSION}/renderdoc/api/app/renderdoc_app.h"

bindgen \
  --whitelist-type 'RENDERDOC*|pRENDERDOC*' \
  --generate-inline-functions \
  --no-prepend-enum-name \
  --impl-debug \
  --impl-partialeq \
  --with-derive-partialeq \
  --with-derive-eq \
  --with-derive-hash \
  "${TEMP_DIR}/renderdoc_app.h" > "${BASE_DIR}/../src/bindings.rs"
