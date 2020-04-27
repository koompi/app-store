#!/bin/bash

which systemfd > /dev/null
[[ $? -eq 1 ]] && cargo install systemfd 
which cargo-watch > /dev/null
[[ $? -eq 1 ]] && cargo install cargo-watch
systemfd --no-pid -s http::3300 -- cargo watch -x run