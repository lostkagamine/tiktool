#!/bin/bash
echo "Ell plus ratio plus $(aspell dump master | shuf | head -n 15 | tee /dev/stderr | sed -z "s/'s//g;s/[[:space:]]*$//g;s/\n/ plus /g")" | cargo r | tee output.mp3 | ffplay -