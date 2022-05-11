---
id: github_app
title: GitHub Integration
---

Squawk works as a CLI tool but can also create comments on GitHub Pull
Requests using the `upload-to-github` subcommand.

Here's a screenshot of an example comment created by `squawk` using the `example.sql` in the repo:

<https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446>

[![squawk pr comment](/img/squawk-pr-comment.png)](https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446)

This document provides instructions for setting up a Squawk GitHub App.

## create a GitHub App

Squawk needs a corresponding GitHub App so it can talk to GitHub.

1. Create the app

   - head over to <https://github.com/settings/apps/new>

   - add an app name & homepage url

   - Uncheck the `active` checkbox under Webhook

   - add permissions

     | name          | kind  | why               |
     | ------------- | ----- | ----------------- |
     | Pull Requests | Write | to comment on PRs |

   - hit create and copy the `App ID` under the "About" section. The page URL should be: https://github.com/settings/apps/$YOUR_APP_NAME

2. Head down the the bottom of the page under the "Private Keys" section and
   hit "Generate a private key"

   The key should automatically download after a couple seconds. Hold onto this key, we'll need it later.

   We now have an `App ID` and a `Private Key`, which is everything we neeed to install the GitHub App.

3. Install the GitHub App & get the Install ID

   Head to <https://github.com/settings/apps/$YOUR_APP_NAME/installations> and click "Install"

   GitHub should have redirected you to the <https://github.com/settings/installations/$INSTALL_ID> page where `$INSTALL_ID` is some number.

   Save this ID for later.

   Now we have our `SQUAWK_GITHUB_APP_ID`, `SQUAWK_GITHUB_PRIVATE_KEY`,
   `SQUAWK_GITHUB_INSTALL_ID`.

   Squawk needs the pull request related values: `SQUAWK_GITHUB_REPO_NAME`,
   `SQUAWK_GITHUB_REPO_OWNER`, and `SQUAWK_GITHUB_PR_NUMBER`.

   Where to find these varies depending how you're running squawk, but for the
   next step I'm assuming you're running Squawk as a CircleCI job.

4. Finding the Pull Request variables

   ### CircleCI

   <https://circleci.com/docs/2.0/env-vars/#built-in-environment-variables>

   `CIRCLE_PULL_REQUEST` has the content we need

   example: `https://github.com/recipeyak/recipeyak/pull/567`

   Now we need to split this to get the repo name, repo owner, and pull
   requeset id.

   With a bit of help from

   ```bash
   # extract org, repo, pr number
   echo "https://github.com/recipeyak/recipeyak/pull/567" | awk -F/ '{print $4 " " $5 " " $7}'

   recipeyak recipeyak 567
   ```

   ```bash
   # store org, repo, and pr number in Squawk's variables.
   SQUAWK_GITHUB_REPO_OWNER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $4}')
   SQUAWK_GITHUB_REPO_NAME=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $5}')
   SQUAWK_GITHUB_PR_NUMBER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $7}')
   ```

5. Conclusion

   Wrapping it all up we should have the following env vars:

   ```bash
   SQUAWK_GITHUB_APP_ID= # fill in with id found in step 5
   SQUAWK_GITHUB_INSTALL_ID= # fill in with id found in step 7
   # downloaded via step 6, your key will have a different name
   SQUAWK_GITHUB_PRIVATE_KEY=$(cat ./cool-bot-name.private-key.pem)
   # can also use the SQUAWK_GITHUB_PRIVATE_KEY_BASE64 instead ^
   SQUAWK_GITHUB_REPO_OWNER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $4}')
   SQUAWK_GITHUB_REPO_NAME=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $5}')
   SQUAWK_GITHUB_PR_NUMBER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $7}')
   ```

   We can pass this into the env before running squawk or we can translate
   them to the command line flag. Whatever's easiest for you.

   An example run will look like the following (assuming the env vars are set):

   ```bash
   squawk upload-to-github example.sql
   ```

   which creates a comment like the following:

   <https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446>

## GitHub Actions authentication

An alternative way to authenticate with GitHub is via a GitHub Actions token.

```
SQUAWK_GITHUB_TOKEN=${{ secrets.GITHUB_TOKEN }}
SQUAWK_GITHUB_REPO_OWNER=$(echo $GITHUB_REPOSITORY | awk -F/ '{print $1}')
SQUAWK_GITHUB_REPO_NAME=$(echo $GITHUB_REPOSITORY | awk -F/ '{print $2}')
SQUAWK_GITHUB_PR_NUMBER=$(echo $GITHUB_REF | awk 'BEGIN { FS = "/" } ; { print $3 }')
squawk upload-to-github example.sql
```