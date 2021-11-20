#!/usr/bin/env bash

if [[ -d /Applications/Eudb_en_free.app ]]; then
    eudicID=$(osascript -e 'id of app "Eudb_en_free"')
elif [[ -d /Applications/Eudic.app ]]; then
    eudicID=$(osascript -e 'id of app "Eudic"')
fi

if [[ -z "$eudicID" ]]; then
osascript <<EOF
display dialog "Please install EuDic"
EOF
   exit
fi

osascript <<EOF
tell application "System Events"
    do shell script "open -b $eudicID"
    tell application id "$eudicID"
        activate
        show dic with word "$1"
    end tell
end tell
EOF

