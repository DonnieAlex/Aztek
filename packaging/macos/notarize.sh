#!/usr/bin/env bash
# Notarizes a signed Aztek.app and staples the ticket to it: packaging/macos/notarize.sh <path/to/Aztek.app>
# Needs an App Store Connect API key: AZTEK_NOTARY_KEY (path to the AuthKey_*.p8), AZTEK_NOTARY_KEY_ID and
# AZTEK_NOTARY_ISSUER. The app must have been signed by sign.sh with AZTEK_SIGN_IDENTITY set.
set -euo pipefail
APP=$1
: "${AZTEK_NOTARY_KEY:?path to the App Store Connect API key (.p8)}"
: "${AZTEK_NOTARY_KEY_ID:?the API key ID}"
: "${AZTEK_NOTARY_ISSUER:?the API key issuer ID}"
KEY=(--key "$AZTEK_NOTARY_KEY" --key-id "$AZTEK_NOTARY_KEY_ID" --issuer "$AZTEK_NOTARY_ISSUER")

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
ditto -c -k --keepParent "$APP" "$WORK/submit.zip"

xcrun notarytool submit "$WORK/submit.zip" "${KEY[@]}" --wait --timeout 2h --output-format plist >"$WORK/result.plist"
ID=$(/usr/libexec/PlistBuddy -c 'Print :id' "$WORK/result.plist")
STATUS=$(/usr/libexec/PlistBuddy -c 'Print :status' "$WORK/result.plist")
if [ "$STATUS" != Accepted ]; then
  echo "Notarization $ID: $STATUS" >&2
  xcrun notarytool log "$ID" "${KEY[@]}" >&2 || true
  exit 1
fi

xcrun stapler staple "$APP"
xcrun stapler validate "$APP"
spctl --assess --type execute --verbose=4 "$APP"
echo "Notarized $APP ($ID)"
