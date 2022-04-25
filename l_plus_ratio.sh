#!/bin/bash

COUNT=10
if [ -n "$1" ]; then
    COUNT=$1
fi

echo "Ell plus ratio plus $(aspell dump master | shuf | head -n $COUNT | tee /dev/stderr | sed -z "s/'s//g;s/[[:space:]]*$//g;s/\n/ plus /g")" | cargo r | tee output.mp3 | ffplay -