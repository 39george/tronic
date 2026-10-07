#!/usr/bin/env bash
# Release helper: checks → bump → changelog → commit → tag → publish → push → GitHub release
#
# Usage:
#   scripts/release.sh                     # release the version currently in Cargo.toml
#   scripts/release.sh patch|minor|major   # bump the version and release
#   scripts/release.sh 0.7.0               # release a specific version
# Flags:
#   --dry-run      run checks and preview the changelog, change nothing
#   --skip-tests   skip fmt/clippy/test
#   -y, --yes      don't ask for confirmation

set -euo pipefail

BRANCH="main"
REMOTE="origin"

DRY_RUN=0
SKIP_TESTS=0
YES=0
BUMP=""
for arg in "$@"; do
  case "$arg" in
  --dry-run) DRY_RUN=1 ;;
  --skip-tests) SKIP_TESTS=1 ;;
  -y | --yes) YES=1 ;;
  -h | --help)
    sed -n '2,13p' "$0"
    exit 0
    ;;
  -*)
    echo "unknown flag: $arg" >&2
    exit 1
    ;;
  *) BUMP="$arg" ;;
  esac
done

if [[ -t 1 ]]; then
  B=$'\e[1m'
  R=$'\e[31m'
  G=$'\e[32m'
  Y=$'\e[33m'
  N=$'\e[0m'
else
  B=""
  R=""
  G=""
  Y=""
  N=""
fi
info() { echo "${B}==>${N} $*"; }
warn() { echo "${Y}warn:${N} $*" >&2; }
die() {
  echo "${R}error:${N} $*" >&2
  exit 1
}
confirm() {
  if ((YES)); then return 0; fi
  local a
  read -rp "$1 [y/N] " a </dev/tty
  [[ "$a" =~ ^[Yy]$ ]]
}

# ---------- Cargo.toml helpers ----------

pkg_field() {
  awk -F'"' -v key="$1" '
    /^\[/ { p = ($0 ~ /^\[package\]/); next }
    p && $0 ~ "^" key "[[:space:]]*=" { print $2; exit }' Cargo.toml
}

write_version() {
  awk -v v="$1" '
    /^\[/ { p = ($0 ~ /^\[package\]/) }
    p && !done && /^version[[:space:]]*=/ { sub(/"[^"]*"/, "\"" v "\""); done = 1 }
    { print }' Cargo.toml >Cargo.toml.tmp
  mv Cargo.toml.tmp Cargo.toml
}

bump_version() {
  local ma mi pa
  IFS=. read -r ma mi pa <<<"${1%%[-+]*}"
  case "$2" in
  major) echo "$((ma + 1)).0.0" ;;
  minor) echo "$ma.$((mi + 1)).0" ;;
  patch) echo "$ma.$mi.$((pa + 1))" ;;
  *)
    [[ "$2" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+].*)?$ ]] || die "invalid version: $2"
    echo "$2"
    ;;
  esac
}

# ---------- environment ----------

cd "$(git rev-parse --show-toplevel)"
[[ -f Cargo.toml ]] || die "Cargo.toml not found in repository root"
for cmd in cargo git git-cliff; do
  command -v "$cmd" >/dev/null || die "$cmd not found"
done

# ---------- git state ----------

info "Checking git state"

cur_branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "(detached)")
[[ "$cur_branch" == "$BRANCH" ]] || die "current branch is $cur_branch, expected $BRANCH"

if ! git diff --quiet || ! git diff --cached --quiet; then
  warn "you have uncommitted changes:"
  git status --short --untracked-files=no
  if ((DRY_RUN)); then die "commit your changes before releasing"; fi
  read -rp "Commit message for them (empty to abort): " msg </dev/tty
  [[ -n "$msg" ]] || die "aborted"
  git add -u
  git commit -m "$msg"
fi

untracked=$(git status --porcelain --untracked-files=normal | sed -n 's/^?? //p')
if [[ -n "$untracked" ]]; then
  warn "untracked files (won't be committed; cargo publish may complain if they're part of the package):"
  echo "$untracked" | sed 's/^/    /'
  confirm "Continue?" || die "aborted"
fi

git fetch "$REMOTE" --tags --quiet
upstream="$REMOTE/$BRANCH"
local_sha=$(git rev-parse HEAD)
remote_sha=$(git rev-parse "$upstream")
base_sha=$(git merge-base HEAD "$upstream")
if [[ "$local_sha" != "$remote_sha" ]]; then
  if [[ "$local_sha" == "$base_sha" ]]; then
    die "branch is behind $upstream — run git pull"
  elif [[ "$remote_sha" != "$base_sha" ]]; then
    die "branch has diverged from $upstream"
  fi
  info "there are unpushed commits — they'll be pushed with the release"
fi

# ---------- version ----------

CRATE=$(pkg_field name)
CUR=$(pkg_field version)
[[ -n "$CUR" ]] || die "failed to read version from [package]"
NEW="$CUR"
if [[ -n "$BUMP" ]]; then NEW=$(bump_version "$CUR" "$BUMP"); fi
TAG="v$NEW"

if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  die "tag $TAG already exists — bump the version (patch/minor/major)"
fi

last_tag=$(git describe --tags --abbrev=0 --match 'v[0-9]*' 2>/dev/null || true)
if [[ -n "$last_tag" ]]; then
  newest=$(printf '%s\n%s\n' "${last_tag#v}" "$NEW" | sort -V | tail -1)
  [[ "$newest" == "$NEW" ]] || die "version $NEW is not greater than the last tag $last_tag"
  if [[ -z "$(git log "$last_tag"..HEAD --oneline)" ]]; then
    warn "no new commits since $last_tag"
    confirm "Release anyway?" || die "aborted"
  fi
fi

info "Releasing ${B}$CRATE $CUR → $NEW${N} (tag $TAG)"

# ---------- checks ----------

if ((!SKIP_TESTS)); then
  info "cargo fmt / clippy / test"
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-features
fi

info "Changelog for $TAG:"
git cliff --unreleased --tag "$TAG" --strip all

if ((DRY_RUN)); then
  info "cargo publish --dry-run"
  cargo publish --dry-run --allow-dirty
  info "${G}dry run OK${N}, nothing was changed"
  exit 0
fi

confirm "Release $TAG?" || die "aborted"

# ---------- bump + changelog ----------

trap 'warn "aborted before commit. To revert: git checkout -- Cargo.toml Cargo.lock CHANGELOG.md"' ERR

if [[ "$NEW" != "$CUR" ]]; then
  write_version "$NEW"
  cargo metadata --format-version 1 >/dev/null # updates the version in Cargo.lock
fi
git cliff --tag "$TAG" --output CHANGELOG.md

info "cargo publish --dry-run"
cargo publish --dry-run --allow-dirty

# ---------- commit + tag ----------

git add Cargo.toml CHANGELOG.md
if git ls-files --error-unmatch Cargo.lock >/dev/null 2>&1; then git add Cargo.lock; fi
if ! git diff --cached --quiet; then
  git commit -m "chore(release): $TAG"
fi
git tag -a "$TAG" -m "Release $TAG"

trap 'warn "aborted after tagging. To revert locally: git tag -d $TAG && git reset --hard HEAD~1"' ERR

# ---------- publish + push ----------

# Publish to crates.io first (irreversible), then push: a failed push is easy
# to retry, while a pushed tag without a published version would have to be
# deleted from the remote.
info "cargo publish"
cargo publish

trap 'warn "crate is published but push failed. Retry: git push --atomic $REMOTE $BRANCH $TAG"' ERR
info "pushing $BRANCH + $TAG"
git push --atomic "$REMOTE" "$BRANCH" "$TAG"
trap - ERR

# ---------- GitHub release ----------

if command -v gh >/dev/null; then
  if confirm "Create GitHub release?"; then
    gh release create "$TAG" --title "$TAG" --notes "$(git cliff --latest --strip all)"
  fi
fi

info "${G}done:${N} https://crates.io/crates/$CRATE/$NEW"
