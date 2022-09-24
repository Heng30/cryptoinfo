#!/bin/bash

curl -X GET \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    https://api-one-wscn.awtmt.com/apiv1/content/lives\?channel\=global-channel\&client=pc\&limit=5\&first_page=true\&accept=live%2Cvip-live

# 下一页的信息，通过cursor来确定
# https://api-one-wscn.awtmt.com/apiv1/content/lives?channel=global-channel&client=pc&cursor=1663983651&limit=20&first_page=false&accept=live%2Cvip-live

