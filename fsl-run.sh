#!/bin/bash

# docker run --rm -it \
# 	--env CARGO_HOME=/frontier/cargo_home \
# 	-v "$(pwd)":/frontier \
#     -w /frontier \
#     frontier-builder \
# 	cargo build --release



docker run --rm -it --name frontier -p 9944:9944 frontier-fsl