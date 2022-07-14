#!/bin/bash
lines=14
target=cryptoinfo-linux
if [ -d "/tmp/$target" ]; then rm -rf /tmp/$target; fi
tail -n +$lines $0 > /tmp/$target.tar.gz
tar -zxvf /tmp/$target.tar.gz -C /tmp
chmod a+x /tmp/$target/install.sh
chmod a+x /tmp/$target/uninstall.sh
/tmp/$target/uninstall.sh > /dev/null
/tmp/$target/install.sh
rm -rf /tmp/$target
rm -f /tmp/$target.tar.gz
exit 0
