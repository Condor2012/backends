#!/bin/sh
BOARD_NAME=$1

curl --request POST \
    --url http://localhost:8080/board \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$BOARD_NAME"'", "tasks": []}'