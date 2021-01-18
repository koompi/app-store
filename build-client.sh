#!/bin/bash

cd client-web &&
    wasm-pack build --release --target web --out-name wasm --out-dir ../static
