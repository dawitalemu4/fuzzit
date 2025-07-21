#!/bin/bash

# strict mode
set -euo pipefail
IFS=$'\n\t'

if [[ -z ${DRY_RUN:-} ]]; then
    PREFIX=""
else
    PREFIX="echo"
fi

# input validation
if [[ -z ${GITHUB_TOKEN:-} ]]; then
    echo "GITHUB_TOKEN environment variable must be set before running." >&2
    exit 1
fi
if [[ $# -ne 1 || $1 == "" ]]; then
    echo "This program requires one argument: the version number, in 'vM.N.P' format." >&2
    exit 1
fi
VERSION=$1

# Change to root of the repo
cd "$(dirname "$0")/.."

# GitHub release

$PREFIX git tag "$VERSION"
# make sure GITHUB_TOKEN is exported, for the benefit of this next command
export GITHUB_TOKEN
$PREFIX make release
# if that was successful, it could have touched build.zig.zon, so revert those
$PREFIX git checkout build.zig.zon

# Homebrew release

URL="https://github.com/dawitalemu4/fuzzit/archive/refs/tags/${VERSION}.tar.gz"
curl -L -o tmp.tgz "$URL"
SHA="$(sha256sum < tmp.tgz | awk '{ print $1 }')"
rm tmp.tgz
HOMEBREW_GITHUB_API_TOKEN="$GITHUB_TOKEN" $PREFIX brew bump-formula-pr --url "$URL" --sha256 "$SHA" fuzzit
