#!/bin/bash

docker build -t demon . && exec docker run -it --rm --name demon demon "$@"
