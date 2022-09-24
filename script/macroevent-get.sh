#!/bin/bash

now_timestamp=`date +%s`
((pre_timestamp = now_timestamp - 24 * 3600 ))
((next_timestamp = now_timestamp + 24 * 3600 ))

curl -X GET \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    https://api-one-wscn.awtmt.com/apiv1/finance/macrodatas\?start\=$pre_timestamp\&end\=$next_timestamp
