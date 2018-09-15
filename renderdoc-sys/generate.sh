#!/bin/bash

set -euo pipefail

readonly HEADER_FILE="$(mktemp -d)/renderdoc_app.h"
curl -o "${HEADER_FILE}" 'https://raw.githubusercontent.com/baldurk/renderdoc/v1.x/renderdoc/api/app/renderdoc_app.h'
bindgen --whitelist-type 'RENDERDOC*|pRENDERDOC*' --rust-target '1.19' "${HEADER_FILE}" > ./src/app.rs
