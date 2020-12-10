#!/bin/bash
cd client &&
    wasm-pack build --release --target web --out-name wasm --out-dir ../static
