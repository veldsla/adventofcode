#!/bin/bash

grep -- "->" input.txt | sed -e 's/ (.*//;' |sort > hasdisk.txt
grep -- "->" input.txt | sed -e 's/.*-> //; s/, /\n/g' |sort > ondisk.txt
diff --unchanged-line-format= --old-line-format= --new-line-format='%L' ondisk.txt hasdisk.txt
