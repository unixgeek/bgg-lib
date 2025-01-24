#!/bin/bash

test -z "${USERNAME}" && echo "USERNAME not set" && exit 1
test -z "${PASSWORD}" && echo "PASSWORD not set" && exit 1

set -Eeuo pipefail

WORK=$(mktemp --directory)

cat >> "${WORK}"/credentials.json <<EOF
{"credentials":{"username":"${USERNAME}","password":"${PASSWORD}"}}
EOF

curl --fail-with-body --silent --show-error --cookie-jar "${WORK}"/cookies.txt --data-binary @"${WORK}"/credentials.json \
    --header 'content-type: application/json' --request POST https://boardgamegeek.com/login/api/v1

curl --fail-with-body --silent --show-error --cookie "${WORK}"/cookies.txt https://boardgamegeek.com/data_dumps/bg_ranks > "${WORK}"/download.html

URL=$(xmllint --html --xpath '//a[text()="Click to Download"]/@href' "${WORK}"/download.html 2> /dev/null | sed 's/href=//;s/"//g;s/&amp;/\&/g' | tr -d ' ')

curl --output "${WORK}"/ids.zip --fail-with-body --silent --show-error  "${URL}"

IDS=$(unzip -p "${WORK}"/ids.zip boardgames_ranks.csv | sed 1d | cut -d ',' -f 1 | tr '\n' ',' | sed 's/,$//')

# There is probably a better way.
echo "[${IDS}]" > "${WORK}"/ids.json
BATCHES=$(jq -c '[.[]] | . as $array | foreach range(0; $array | length; 20) as $i ([];
  $array[$i : ($i + 20)])' "${WORK}"/ids.json)

COUNT=0
for B in $BATCHES; do
  NEXT=$(echo "${B}" | jq -r -c '.' | tr -d '[' | tr -d ']')
  curl -o ${COUNT}.xml --no-progress-meter -H 'user-agent: one-time-for-fun' "https://boardgamegeek.com/xmlapi2/thing?id=${NEXT}&stats=1"
  sleep 2
  COUNT=$((COUNT+1))
done

# Use xmllint --xpath /items/item/@id, etc., to check for max values.