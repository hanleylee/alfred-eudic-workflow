#!/usr/bin/env bash

Eudic_ID=$(osascript -e 'id of app "Eudb_en_free"' 2>/dev/null) || \
    Eudic_ID=$(osascript -e 'id of app "Eudb_en"' 2>/dev/null) || \
    Eudic_ID=$(osascript -e 'id of app "Eudic"' 2>/dev/null)

if [[ -z "$Eudic_ID" ]]; then
osascript <<EOF
display dialog "Please install EuDic"
EOF
   exit
fi

osascript <<EOF
tell application "System Events"
    do shell script "open -b $Eudic_ID"
    tell application id "$Eudic_ID"
        activate
        show dic with word "$1"
    end tell
end tell
EOF

