#!/bin/bash

set -eu

EXPORT_DIR=buf_exported

mkdir -p ${EXPORT_DIR}

# Export all proto-files
buf export -o ${EXPORT_DIR}

# List proto files to compile (captured in build.rs)
buf ls-files > ${EXPORT_DIR}/protos.txt
