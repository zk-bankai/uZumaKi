#!/usr/bin/env bash

set -eo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
OUTPUT_DIR="$(pwd)"

while true; do
  case "$1" in
    -o | --output-dir ) OUTPUT_DIR="$2"; shift 2 ;;
    * ) break ;;
  esac
done

mkdir -p "${OUTPUT_DIR}"

STONE_DIR="${SCRIPT_DIR}/../dependencies/stone-prover"

STONE_GIT_TAG=$(cat .git/modules/dependencies/stone-prover/HEAD)
IMAGE_NAME="stone-prover-build:${STONE_GIT_TAG}"
ARCH_NAME=$(uname -m)

BUILD_FLAGS=""
if [ "${ARCH_NAME}" == "arm64" ]; then BUILD_FLAGS="--build-arg CMAKE_ARGS=-DNO_AVX=1"; fi

docker build -t "${IMAGE_NAME}" ${BUILD_FLAGS} "${STONE_DIR}" --network host

CONTAINER=$(docker create "${IMAGE_NAME}")
docker cp -L "${CONTAINER}:/bin/cpu_air_prover" "${OUTPUT_DIR}"
docker cp -L "${CONTAINER}:/bin/cpu_air_verifier" "${OUTPUT_DIR}"

docker rm "${CONTAINER}"