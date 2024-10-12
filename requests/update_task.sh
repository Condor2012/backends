#!/bin/sh
BOARD_ID=$1
TASK_NAME=$2
TASK_NEW_NAME=$3

curl --request PUT \
    --url "http://localhost:8080/board/$BOARD_ID/task/$TASK_NAME" \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$TASK_NEW_NAME"'", "description": "'"Description of $TASK_NAME"'"}'