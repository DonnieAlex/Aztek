#!/usr/bin/env python3
"""Picks the issue the issue bot works on next, or none, and marks it auto:working.

Run by .github/workflows/issue-bot.yml. An issue is taken when:
  - it was named by hand (workflow_dispatch with an issue number), or labelled auto by someone with write access;
  - it waits on a question the bot asked (auto:waiting) and its author, or someone on the allow list, has answered since;
  - it is new: opened on or after "since" by someone on the allow list, or labelled auto, with no auto:* or skip label.
Answers first, then new issues, oldest first. Nothing new starts while max_open_prs of the bot's pull requests wait.
Writes number= and mode= (new or followup) to $GITHUB_OUTPUT; number is empty when there's nothing to do.
"""
import json
import os
import subprocess
import sys

REPO = os.environ.get("GITHUB_REPOSITORY", "srlegrand/Aztek")
CONF = json.load(open(".github/issue-bot.json"))
ALLOW = {u.lower() for u in CONF["allow"]}
ASK_MARK = "<!-- issue-bot:ask -->"


def gh_lines(path):
    out = subprocess.run(["gh", "api", "--paginate", path, "--jq", ".[]"], check=True, capture_output=True, text=True).stdout
    return [json.loads(l) for l in out.splitlines() if l.strip()]


def gh(*args):
    return subprocess.run(["gh", *args], check=False, capture_output=True, text=True)


def labels(issue):
    return {l["name"] for l in issue.get("labels", [])}


def answered(issue):
    """A comment from the author or the allow list after the bot's last question."""
    comments = gh_lines(f"repos/{REPO}/issues/{issue['number']}/comments?per_page=100")
    asked = [i for i, c in enumerate(comments) if ASK_MARK in (c.get("body") or "")]
    if not asked:
        return False
    author = issue["user"]["login"].lower()
    later = comments[asked[-1] + 1:]
    return any(c["user"]["login"].lower() in ALLOW | {author} and not c["user"]["login"].endswith("[bot]") for c in later)


def mode_of(issue):
    return "followup" if "auto:waiting" in labels(issue) else "new"


def done(number, mode):
    with open(os.environ.get("GITHUB_OUTPUT", "/dev/stdout"), "a") as f:
        f.write(f"number={number}\nmode={mode}\n")
    if number:
        gh("issue", "edit", str(number), "--repo", REPO, "--add-label", "auto:working", "--remove-label", "auto:waiting")
        print(f"Taking #{number} ({mode})")
    else:
        print("Nothing to do")
    sys.exit(0)


def main():
    event = os.environ.get("EVENT_NAME", "")
    named = os.environ.get("ISSUE_INPUT", "").strip().lstrip("#")
    if event == "issues":
        if os.environ.get("LABEL") != "auto":
            done("", "")
        named = os.environ.get("ISSUE_NUMBER", "")
    if named:
        issue = json.loads(gh("api", f"repos/{REPO}/issues/{named}").stdout)
        if issue.get("state") != "open" or "pull_request" in issue or "auto:working" in labels(issue):
            done("", "")
        done(named, mode_of(issue))

    open_prs = [p for p in gh_lines(f"repos/{REPO}/pulls?state=open&per_page=100") if p["head"]["ref"].startswith("auto/issue-")]
    if len(open_prs) >= CONF["max_open_prs"]:
        print(f"{len(open_prs)} of the bot's pull requests are waiting; nothing new starts")
        done("", "")
    issues = sorted((i for i in gh_lines(f"repos/{REPO}/issues?state=open&per_page=100") if "pull_request" not in i), key=lambda i: i["number"])
    skip = set(CONF["skip_labels"])
    for i in issues:
        if "auto:waiting" in labels(i) and not labels(i) & skip and answered(i):
            done(i["number"], "followup")
    for i in issues:
        ls = labels(i)
        if ls & skip or any(l.startswith("auto:") for l in ls):
            continue
        trusted = i["user"]["login"].lower() in ALLOW and i["created_at"][:10] >= CONF["since"]
        if trusted or "auto" in ls:
            done(i["number"], "new")
    done("", "")


if __name__ == "__main__":
    main()
