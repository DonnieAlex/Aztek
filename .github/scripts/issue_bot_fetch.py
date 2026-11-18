#!/usr/bin/env python3
"""Writes one issue to a folder, the way the bot reads it: issue.md (title, author, dates, labels, body, every comment),
the pictures and files attached to it as 01.png, 02.mp4 and so on with the links pointing at them, and meta.json.

    python3 .github/scripts/issue_bot_fetch.py <number> <folder>

HTML comments and invisible characters are taken out of the text: what the reporter wrote is shown, nothing hidden.
"""
import json
import os
import re
import subprocess
import sys
import urllib.request

REPO = os.environ.get("GITHUB_REPOSITORY", "srlegrand/Aztek")
ASK_MARK = "<!-- issue-bot:ask -->"
HIDDEN = re.compile(r"<!--.*?-->|[​-‏ -‮⁠-⁤﻿]", re.S)
LINK = re.compile(r"https://(?:github\.com/user-attachments/(?:assets|files)/[\w./-]+|user-images\.githubusercontent\.com/[\w./%-]+|private-user-images\.githubusercontent\.com/[^\s\"')]+)")
TYPES = {"image/png": ".png", "image/jpeg": ".jpg", "image/gif": ".gif", "image/webp": ".webp", "video/mp4": ".mp4", "video/quicktime": ".mov", "application/zip": ".zip", "text/plain": ".txt"}


def gh_json(path, paginate=False):
    args = ["gh", "api", path] + (["--paginate", "--jq", ".[]"] if paginate else [])
    out = subprocess.run(args, check=True, capture_output=True, text=True).stdout
    return [json.loads(l) for l in out.splitlines() if l.strip()] if paginate else json.loads(out)


def date(s):
    return f"{s[8:10]}/{s[5:7]}/{s[0:4]} {s[11:16]} UTC" if s else ""


def main():
    number, folder = sys.argv[1], sys.argv[2]
    os.makedirs(folder, exist_ok=True)
    issue = gh_json(f"repos/{REPO}/issues/{number}")
    comments = gh_json(f"repos/{REPO}/issues/{number}/comments?per_page=100", paginate=True)
    files, n = {}, 0

    def keep(url):
        nonlocal n
        if url in files:
            return files[url]
        data = kind = None
        # Without a token first, as a browser fetches them; then with the workflow's token.
        for auth in ({}, {"Authorization": "token " + os.environ.get("GH_TOKEN", "")}):
            try:
                req = urllib.request.Request(url, headers={"User-Agent": "aztek-issue-bot", **auth})
                with urllib.request.urlopen(req, timeout=60) as r:
                    data = r.read(60 * 1024 * 1024)
                    kind = r.headers.get_content_type()
                break
            except Exception as e:
                err = e
        if data is None:
            print(f"couldn't download {url}: {err}")
            files[url] = url
            return url
        n += 1
        name = f"{n:02d}{TYPES.get(kind, os.path.splitext(url.split('?')[0])[1] or '.bin')}"
        with open(os.path.join(folder, name), "wb") as f:
            f.write(data)
        files[url] = name
        return name

    def clean(text):
        text = HIDDEN.sub("", (text or "").replace("\r\n", "\n"))
        return LINK.sub(lambda m: keep(m.group(0)), text)

    labels = [l["name"] for l in issue.get("labels", [])]
    lines = [
        f"# #{number} {clean(issue['title'])}",
        "",
        f"- URL: {issue['html_url']}",
        f"- Author: {issue['user']['login']}",
        f"- Opened: {date(issue['created_at'])}, updated: {date(issue['updated_at'])}",
        f"- Labels: {', '.join(labels) or 'none'}",
        "",
        clean(issue.get("body")),
    ]
    asks = 0
    for i, c in enumerate(comments, 1):
        mine = ASK_MARK in (c.get("body") or "")
        asks += mine
        who = c["user"]["login"] + (" (the bot's question)" if mine else "")
        lines += ["", "", f"## Comment {i} by {who}, {date(c['created_at'])}", "", clean(c.get("body"))]
    with open(os.path.join(folder, "issue.md"), "w") as f:
        f.write("\n".join(lines).rstrip() + "\n")
    meta = {"number": int(number), "title": issue["title"], "author": issue["user"]["login"], "labels": labels, "asks": asks, "attachments": sorted(v for v in files.values() if not v.startswith("http"))}
    with open(os.path.join(folder, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1)
    print(f"#{number}: {len(comments)} comments, {len(meta['attachments'])} attachments, {asks} questions asked")


if __name__ == "__main__":
    main()
