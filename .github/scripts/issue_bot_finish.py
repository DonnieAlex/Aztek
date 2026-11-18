#!/usr/bin/env python3
"""After the bot has worked on an issue: checks what it left in .issue/out/result.json, turns its changes into
.issue/out/change.patch, and for a fix runs the tests itself. Anything wrong turns the decision into needs-simon, with
the reason in the summary. Nothing here can push or comment: the publish job does that from what this leaves.

    python3 .github/scripts/issue_bot_finish.py <base sha>
"""
import json
import os
import re
import subprocess
import sys

OUT = ".issue/out"
DECISIONS = {"fixed", "ask", "not-reference", "close", "needs-simon"}
PROTECTED = (".github/", ".claude/")
WRITER = re.compile(r"\b(Claude|Anthropic|AI|LLM|bot|automated|automation|language model)\b")


def run(cmd, **kw):
    return subprocess.run(cmd, shell=isinstance(cmd, str), capture_output=True, text=True, **kw)


def main():
    base = sys.argv[1]
    os.makedirs(OUT, exist_ok=True)
    path = os.path.join(OUT, "result.json")
    try:
        result = json.load(open(path))
    except Exception as e:
        result = {"decision": "needs-simon", "summary": f"The bot left no readable result ({e})."}
    notes = []
    decision = result.get("decision")
    if decision not in DECISIONS:
        notes.append(f"Unknown decision {decision!r}.")
        decision = "needs-simon"
    if decision in {"ask", "not-reference", "close"} and not (result.get("reply") or "").strip():
        notes.append("No reply was written.")
        decision = "needs-simon"

    # Posted as Simon: no em dashes, and nothing about who or what wrote it.
    reply = result.get("reply") or ""
    reply = re.sub(r"\s*[\u2014\u2013]\s*", ", ", reply)
    result["reply"] = reply
    named = WRITER.findall(reply)
    if named and decision != "needs-simon":
        notes.append("The reply mentions " + ", ".join(sorted(set(named))) + "; left for Simon to word.")
        decision = "needs-simon"

    # Everything changed since the run started, committed or not; .issue/ is ignored by git.
    run("git add -A")
    diff = run(["git", "diff", "--cached", "--binary", base]).stdout
    touched = run(["git", "diff", "--cached", "--name-only", base]).stdout.split()
    with open(os.path.join(OUT, "change.patch"), "w") as f:
        f.write(diff)
    if decision == "fixed":
        if not diff.strip():
            notes.append("The decision was fixed but nothing changed.")
            decision = "needs-simon"
        elif any(p.startswith(PROTECTED) for p in touched):
            notes.append("The change touches .github/ or .claude/, which the bot may not change: " + ", ".join(p for p in touched if p.startswith(PROTECTED)))
            decision = "needs-simon"
        elif not (result.get("commit") or "").strip():
            notes.append("No commit message was written.")
            decision = "needs-simon"
    elif diff.strip() and decision != "needs-simon":
        notes.append("Files were changed but the decision isn't fixed; the changes are left out.")

    if decision == "fixed":
        gates = ["cargo test --workspace"]
        if any(p.startswith("app/") for p in touched):
            gates.append("tools/check_app.sh")
        # ISSUE_BOT_GATES="cmd;cmd" replaces them, for trying this out.
        if os.environ.get("ISSUE_BOT_GATES") is not None:
            gates = [g for g in os.environ["ISSUE_BOT_GATES"].split(";") if g.strip()]
        for g in gates:
            print(f"gate: {g}", flush=True)
            r = run(g)
            if r.returncode != 0:
                tail = "\n".join((r.stdout + r.stderr).splitlines()[-40:])
                notes.append(f"`{g}` failed:\n\n```\n{tail}\n```")
                decision = "needs-simon"
                break

    result["decision"] = decision
    if notes:
        result["summary"] = ((result.get("summary") or "") + "\n\n" + "\n\n".join(notes)).strip()
    result["touched"] = touched
    with open(path, "w") as f:
        json.dump(result, f, indent=1)
    print(f"decision: {decision}")
    for n in notes:
        print(n)


if __name__ == "__main__":
    main()
