#!/bin/bash

cd client-web &&
    wasm-pack build --release --target web --out-name wasm --out-dir ../static

rollup ./main.js --format iife --file ./pkg/bundle.js
python -m http.server 8000