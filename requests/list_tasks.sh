#!/bin/sh
BOARD_ID=$1

curl --request GET \
    --url "http://localhost:8080/board/$BOARD_ID/task" \
    --header "Content-Type: application/json"