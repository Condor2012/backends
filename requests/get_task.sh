#!/bin/sh
BOARD_ID=$1
TASK_NAME=$2

curl --request GET \
    --url "http://localhost:8080/board/$BOARD_ID/task/$TASK_NAME" \
    --header "Content-Type: application/json"