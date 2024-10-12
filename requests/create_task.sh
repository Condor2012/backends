#!/bin/sh
BOARD_ID=$1
TASK_NAME=$2

curl --request POST \
    --url "http://localhost:8080/board/$BOARD_ID/task" \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$TASK_NAME"'", "description": "'"Description of $TASK_NAME"'"}'