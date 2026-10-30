#!/usr/bin/env bash
# sync.sh - Sync the current branch to both remotes (gitlab + github).
#
# 1. Aborts if there are uncommitted changes.
# 2. Pulls from the primary remote (gitlab), fast-forward only.
# 3. Pushes to every remote, reporting the result of each.
set -euo pipefail

PRIMARY="gitlab"
REMOTES=("gitlab" "github")

cd "$(git rev-parse --show-toplevel)"

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [ "$BRANCH" = "HEAD" ]; then
    echo "error: detached HEAD - check out a branch first." >&2
    exit 1
fi

# 1. Everything must be committed (untracked files are ignored).
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "error: you have uncommitted changes. Commit them first:" >&2
    git status --short >&2
    exit 1
fi

# 2. Pull from the primary remote; stop if histories diverged.
echo "Pulling $BRANCH from $PRIMARY..."
git pull --ff-only "$PRIMARY" "$BRANCH"

# 3. Push to every remote; try all of them even if one fails.
status=0
for remote in "${REMOTES[@]}"; do
    echo "Pushing $BRANCH to $remote..."
    if git push "$remote" "$BRANCH"; then
        echo "  ok: $remote"
    else
        echo "  FAILED: $remote" >&2
        status=1
    fi
done

exit $status
