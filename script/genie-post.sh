#!/bin/bash

curl -X POST -H "content-type: application/json" \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    -d '{"volumeType":"eth","timePeriod":"twenty_four_hours"}' \
    https://genie-production-api.herokuapp.com/collections/trending
