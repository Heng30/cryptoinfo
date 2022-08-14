#!/bin/bash

curl -X POST -H "content-type: application/json" \
    -H "x-api-key: iMHRYlpIXs3zfcBY1r3iKLdqS2YUuOUs" \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    -H "origin: https://www.gem.xyz" \
    -H "referer: https://www.gem.xyz/" \
    -d '{"sort":{"stats.one_day_volume":-1},"limit":100,"fields":{"name":1,"stats":1}}' \
    https://search.gemlabs.xyz/collections
