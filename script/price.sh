#!/bin/bash

curl -X GET \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    https://api.alternative.me/v1/ticker/?limit=100
