# Issue bot

`.github/workflows/issue-bot.yml` works through the issues one at a time, every hour at 23 minutes past, with Claude
Code (`anthropics/claude-code-action`) following `.claude/skills/fix-issue/SKILL.md`. For each issue it decides one of:

| Decision | What happens |
| --- | --- |
| fixed | One commit on `auto/issue-N` and a pull request. The reply sits in the pull request's description; merging posts it on the issue and closes it (`issue-bot-close.yml`). Edit the reply there first if it needs it. |
| ask | A comment asking the reporter to run a script in the reference and paste what it prints; `auto:waiting`. When the reporter (or someone on the allow list) answers, the next run takes it up again. |
| not-reference | A comment showing the reference does the same; `not-reference`, left open for you to close. |
| close | A comment, and the issue closed: thanks, answered questions, duplicates. |
| needs-srlegrand | Nothing posted; `needs-srlegrand`. What it got to, if anything, is on `auto/draft-issue-N`. The run's summary says why. |

## An issue labelled needs-srlegrand

The bot stopped and posted nothing. To pick it up:

1. Read why: Actions, Issue bot, the run for that issue; its summary says what it found, what stopped it and what it
   would do.
2. If the summary names `auto/draft-issue-N`, look at what it got to: `git fetch origin auto/draft-issue-N` and
   `git diff main...FETCH_HEAD`.
3. Then one of:
   - Finish it yourself: fix it (from the draft or not), reply on the issue, close it, and remove the label.
   - Hand it back: add a comment with what it lacked (your decision, the reference's output), remove
     `needs-srlegrand`, then add `auto`. The next run reads every comment, yours included.
   - Close it, if it shouldn't be done.
4. Delete `auto/draft-issue-N` once it's settled: `git push origin --delete auto/draft-issue-N`.

## Which issues

From `.github/issue-bot.json`:

- New issues opened on or after `since` by someone on `allow`.
- Any open issue you label `auto`: it starts straight away.
- A question answered by the issue's author.
- By hand: Actions, Issue bot, Run workflow, with an issue number.

Never: pull requests, issues with a label in `skip_labels` or an `auto:` label of their own. Nothing new starts while
`max_open_prs` of the bot's pull requests wait for you. After `max_asks` unanswered questions an issue goes to
`needs-srlegrand`.

## Safety

The issue's text goes to Claude as data, with HTML comments and invisible characters taken out. Only issues from the
allow list, or that you labelled, are taken. The job Claude runs in has a token that can only read; it may not edit
`.github/` or `.claude/`. A separate job with no Claude in it checks the result, runs `cargo test --workspace` (and
`tools/check_app.sh` for changes under `app/`) itself, refuses changes under `.github/` or `.claude/`, then pushes,
opens the pull request and comments. Nothing reaches `main` until you merge.

## Setting it up

1. Run `claude setup-token` and put the token in the repository's secrets:
   `gh secret set CLAUDE_CODE_OAUTH_TOKEN --repo srlegrand/Aztek`.
2. Make the labels (the commands are below).
3. Optional, for replies posted as you rather than as github-actions: a fine-grained personal access token with Issues
   read and write on this repository only, as the secret `AZTEK_REPLY_TOKEN`. Only the scripts that post replies see
   it, never Claude.
4. Optional, for the commits to carry your name: the repository variables `AZTEK_COMMIT_NAME` and
   `AZTEK_COMMIT_EMAIL`.
5. Settings, Actions, General: "Allow GitHub Actions to create and approve pull requests" on (the patch workflow needs
   it too).

```
gh label create auto --repo srlegrand/Aztek --color 0e8a16 --description "Let the issue bot take this"
gh label create auto:working --repo srlegrand/Aztek --color fbca04 --description "The issue bot is on it"
gh label create auto:waiting --repo srlegrand/Aztek --color c5def5 --description "Waiting for an answer from the reporter"
gh label create auto:pr --repo srlegrand/Aztek --color 1d76db --description "A fix waits in a pull request"
gh label create auto:failed --repo srlegrand/Aztek --color b60205 --description "The issue bot's run failed"
gh label create needs-srlegrand --repo srlegrand/Aztek --color d93f0b --description "Left for srlegrand"
gh label create not-reference --repo srlegrand/Aztek --color cccccc --description "The reference does the same"
```

To try it on one issue: label it `auto`, or run the workflow by hand with its number. To stop it: disable the workflow
in Actions.

Pull requests the bot opens don't start the Build workflow (GitHub doesn't run workflows on what the workflow token
does); the bot's job has run the tests already, and merging builds and releases as any push to `main` does.
