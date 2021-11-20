#!/usr/bin/env bash

eudicID=$(osascript -e 'id of app "Eudic"')

osascript <<EOF
tell application "System Events"
    do shell script "open -b $eudicID"
    tell application id "$eudicID"
        activate
        show dic with word "$1"
    end tell
end tell
EOF

