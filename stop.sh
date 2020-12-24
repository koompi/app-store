#!/bin/bash
kill -9 $(lsof -t -i:4000)
