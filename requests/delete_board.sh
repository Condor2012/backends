#!/bin/sh
BOARD_ID=$1

curl --request DELETE \
    --url "http://localhost:8080/board/$BOARD_ID" \
    --header "Content-Type: application/json"