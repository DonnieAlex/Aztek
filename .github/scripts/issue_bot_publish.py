#!/usr/bin/env python3
"""Carries out the bot's decision on an issue, from .issue/out/ (result.json, change.patch) and .issue/meta.json:

  fixed          the change as one commit on auto/issue-N and a pull request; the reply waits in the pull request and
                 is posted, and the issue closed, when it is merged (.github/workflows/issue-bot-close.yml)
  ask            the reply posted as a question (a script to run in the reference, say); auto:waiting until answered
  not-reference  the reply posted; labelled not-reference and left open for srlegrand to close
  close          the reply posted and the issue closed (thanks, questions answered, duplicates)
  needs-srlegrand    labelled needs-srlegrand; nothing posted

Replies go out with REPLY_TOKEN (srlegrand's own token when one is set, else the workflow's). The bot's own process never
sees this token or a token that can push.
"""
import json
import os
import subprocess
import sys

REPO = os.environ.get("GITHUB_REPOSITORY", "srlegrand/Aztek")
NUMBER = os.environ["NUMBER"]
BASE = os.environ["BASE_SHA"]
BRANCH = f"auto/issue-{NUMBER}"
ASK_MARK = "<!-- issue-bot:ask -->"
CONF = json.load(open(".github/issue-bot.json"))


def run(*cmd, token=None, check=True):
    env = dict(os.environ)
    if token:
        env["GH_TOKEN"] = token
    r = subprocess.run(cmd, capture_output=True, text=True, env=env)
    if check and r.returncode != 0:
        sys.exit(f"{' '.join(cmd[:3])}: {r.stderr.strip()}")
    return r


def labels(add=(), remove=("auto:working",)):
    args = ["gh", "issue", "edit", NUMBER, "--repo", REPO]
    for l in add:
        args += ["--add-label", l]
    for l in remove:
        args += ["--remove-label", l]
    run(*args, check=False)


def reply(text, token):
    run("gh", "issue", "comment", NUMBER, "--repo", REPO, "--body", text, token=token)


def summary(text):
    with open(os.environ.get("GITHUB_STEP_SUMMARY", "/dev/stdout"), "a") as f:
        f.write(text + "\n")


def main():
    result = json.load(open(".issue/out/result.json"))
    meta = json.load(open(".issue/meta.json"))
    token = os.environ.get("REPLY_TOKEN") or os.environ["GH_TOKEN"]
    decision = result["decision"]
    text = (result.get("reply") or "").strip()
    if decision == "ask" and meta.get("asks", 0) >= CONF["max_asks"]:
        result["summary"] = (result.get("summary") or "") + f"\n\nAlready asked {meta['asks']} times; left for srlegrand."
        decision = "needs-srlegrand"
    summary(f"## #{NUMBER}: {decision}\n\n{result.get('summary') or ''}\n")

    if decision == "fixed":
        message = result["commit"].strip()
        subject = message.splitlines()[0]
        name = os.environ.get("COMMIT_NAME") or "github-actions[bot]"
        email = os.environ.get("COMMIT_EMAIL") or "41898282+github-actions[bot]@users.noreply.github.com"
        run("git", "checkout", "-q", "-B", BRANCH, BASE)
        if os.path.getsize(".issue/out/change.patch"):
            run("git", "apply", "--index", "--whitespace=nowarn", ".issue/out/change.patch")
        run("git", "-c", f"user.name={name}", "-c", f"user.email={email}", "commit", "-q", "-m", message)
        run("git", "push", "-q", "--force", "origin", BRANCH)
        body = (
            f"For #{NUMBER}.\n\n{result.get('summary') or ''}\n\n"
            "The reply below is posted on the issue, and the issue closed, when this is merged. Edit it here first if it "
            "needs changing.\n\n<!-- reply -->\n" + text + "\n<!-- /reply -->\n"
        )
        made = run("gh", "pr", "create", "--repo", REPO, "--base", "main", "--head", BRANCH, "--title", subject, "--body", body, check=False)
        if made.returncode != 0 and "already exists" in made.stderr:
            run("gh", "pr", "edit", BRANCH, "--repo", REPO, "--title", subject, "--body", body)
        elif made.returncode != 0:
            sys.exit(f"gh pr create: {made.stderr.strip()}")
        labels(add=["auto:pr"])
    elif decision == "ask":
        reply(text + "\n\n" + ASK_MARK, token)
        labels(add=["auto:waiting"])
    elif decision == "not-reference":
        reply(text, token)
        labels(add=["not-reference"])
    elif decision == "close":
        reply(text, token)
        reason = "not planned" if result.get("close_reason") == "not planned" else "completed"
        run("gh", "issue", "close", NUMBER, "--repo", REPO, "--reason", reason, token=token)
        labels()
    else:
        # What it got to, on a branch of its own with no pull request, for a look.
        if os.path.getsize(".issue/out/change.patch"):
            draft = f"auto/draft-issue-{NUMBER}"
            run("git", "checkout", "-q", "-B", draft, BASE)
            if run("git", "apply", "--index", "--whitespace=nowarn", ".issue/out/change.patch", check=False).returncode == 0:
                message = (result.get("commit") or f"Unfinished work on #{NUMBER}").strip()
                run("git", "-c", "user.name=github-actions[bot]", "-c", "user.email=41898282+github-actions[bot]@users.noreply.github.com", "commit", "-q", "-m", message)
                run("git", "push", "-q", "--force", "origin", draft)
                summary(f"Unfinished change on the branch `{draft}`.")
        labels(add=["needs-srlegrand"])


if __name__ == "__main__":
    main()
