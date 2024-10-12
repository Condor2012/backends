#!/bin/sh
curl --request GET \
    --url http://localhost:8080/board \
    --header "Content-Type: application/json"