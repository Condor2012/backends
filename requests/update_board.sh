#!/bin/sh
BOARD_ID=$1
BOARD_NAME=$2

curl --request PUT \
    --url "http://localhost:8080/board/$BOARD_ID" \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$BOARD_NAME"'", "tasks": []}'