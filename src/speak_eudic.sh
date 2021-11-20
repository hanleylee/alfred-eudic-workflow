#!/usr/bin/env bash
eudicID=$(osascript -e 'id of app "Eudic"')

osascript <<EOF
tell application id "$eudicID"
    speak word with word "$1"
end tell
EOF

