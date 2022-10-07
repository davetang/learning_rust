#!/usr/bin/env bash

set -euo pipefail

if [[ ! -x $(command -v docker) ]]; then
   >&2 echo Could not find docker
   exit 1
fi

cd $(dirname $0)/..

image=rust:1.64.0
container_name=learning_rust

if docker ps -a --filter name=${container_name} --filter status=exited | grep ${container_name} > /dev/null; then
   >&2 echo Restarting ${container_name}
   docker start ${container_name} > /dev/null
   docker exec -it ${container_name} /bin/bash
elif docker ps -a --filter name=${container_name} | grep ${container_name} > /dev/null; then
   >&2 echo Running /bin/bash interactively for ${container_name}
   docker exec -it ${container_name} /bin/bash
else
   >&2 echo Starting ${container_name}
   docker run \
      --name ${container_name} \
      -it \
      -u $(id -u):$(id -g) \
      -v $(pwd):/work \
      ${image} \
      /bin/bash
fi

