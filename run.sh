#!/bin/bash

action=$1

usage() {
  echo "Usage : $0 [action](gen|start|stop)"
  echo
  echo "action :"
  echo   "* gen: Generate slides"
  echo   "* start: Generate slides and start presentation web server"
}

if ! [[ $action =~ ^(gen|start)$ ]]; then
  usage
  exit 2
fi

if [[ $action == "gen" || $action == "start" ]]; then
    echo "Generate slides"
    podman container run --rm -v $(pwd):/documents  cds-bdx-docker.repo-nf.groupeonepoint.com/gop-asciidoctor:latest gop-reveal slides/index.adoc
fi

if [[ $action == "start" ]]; then
    echo "Run presentation web server on http://localhost:8080"
    podman run --rm -p 8080:80 -v $(pwd)/slides:/usr/share/nginx/html --name docker-slides nginx
fi
