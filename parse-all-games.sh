#!/bin/bash -e

XML_FILES=$(ls -- *.xml)
for XF in $XML_FILES; do
  echo "${XF}"
  cargo run --quiet --bin parse "${XF}" >> parsed.txt
done
