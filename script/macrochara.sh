#!/bin/bash

curl -X GET \
    -H "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36" \
    -H "accept: */*" \
    https://api-ddc-wscn.awtmt.com/market/real\?fields=prod_name\%2Cpreclose_px\%2Clast_px\%2Cpx_change\%2Cpx_change_rate\%2Cprice_precision\&prod_code=000001.SS\%2CDXY.OTC\%2CUS10YR.OTC\%2CUSDCNH.OTC\%2C399001.SZ\%2C399006.SZ

