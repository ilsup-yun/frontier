#!/bin/bash

docker run --rm -it \
	--env CARGO_HOME=/frontier/cargo_home \
	-v "$(pwd)":/frontier \
    -w /frontier \
    frontier-builder \
	cargo build --release


docker build -f 'fsl/Dockerfile.build' -t "frontier-fsl" .
