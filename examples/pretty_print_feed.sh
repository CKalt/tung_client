#!/bin/sh
if [ "$#" -lt "1" ]; then
    echo "usage: $0 <file_name>"
    echo " ex. $ $0 example_feed.log"
    exit 1
fi
FILE_NAME=$1
grep "Received:" ${FILE_NAME} | \
    sed 's/Received: //' | 
    while IFS='$\n' read -r line; do \
        echo $line | json_pp 
    done
