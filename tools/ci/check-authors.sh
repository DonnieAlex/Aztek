#!/usr/bin/env bash
# Fails when a commit in the range is authored or committed by an AI tool or under a placeholder identity (a name or
# address left as template text, or an address with no @ in it), or carries AI attribution in the message:
# tools/ci/check-authors.sh <base> <head>. CI runs it on every push and pull request; tools/git-hooks/pre-push runs it
# before a push leaves the machine.
set -euo pipefail
base=$1 head=$2
range="$head"
[ -n "$base" ] && ! [[ "$base" =~ ^0+$ ]] && range="$base..$head"

bad_id='(noreply@anthropic\.com|@anthropic\.com|^claude$|^claude\[bot\]|claude-code)'
placeholder='(from above|your name|your email|^name$|^email$|^unknown$|^$)'
bad_msg='(co-authored-by:.*(claude|anthropic)|claude\.ai/code|generated with \[?claude)'

fail=0
while IFS=$'\t' read -r sha an ae cn ce; do
  for id in "$an" "$ae" "$cn" "$ce"; do
    if grep -Eqi "$bad_id" <<<"$id"; then
      echo "::error::$sha is authored or committed as '$an <$ae>' / '$cn <$ce>': commit as yourself."
      fail=1
      break
    fi
  done
  if grep -Eqi "$placeholder" <<<"$an" || grep -Eqi "$placeholder" <<<"$cn" || ! [[ "$ae" == *@* && "$ce" == *@* ]]; then
    echo "::error::$sha is authored or committed as '$an <$ae>' / '$cn <$ce>', a placeholder: set git's user.name and user.email to your own."
    fail=1
  fi
  if git log -1 --format=%B "$sha" | grep -Eqi "$bad_msg"; then
    echo "::error::$sha carries AI attribution in its message: take the trailer or link out."
    fail=1
  fi
done < <(git log --format=$'%H\t%an\t%ae\t%cn\t%ce' "$range")
[ "$fail" = 0 ] && echo "Authors checked: $range"
exit "$fail"
