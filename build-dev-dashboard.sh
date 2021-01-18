#!/bin/bash

cd dev-dashboard &&
    wasm-pack build --release --target web --out-name wasm --out-dir ../static-dev-dashboard
