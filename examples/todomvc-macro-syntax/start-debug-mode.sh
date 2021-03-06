#!/bin/bash

set -v

./bootstrap.sh


wasm-pack build --target web --dev -- --features "console_error_panic_hook"

basic-http-server ./ -a 0.0.0.0:6001
