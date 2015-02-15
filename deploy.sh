#!/usr/bin/env bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

mkdir _site
cd _site
git init
git config user.name "Paho Lurie-Gregg"
git config user.email "paho@paholg.com"

git remote add upstream "https://$GH_TOKEN@github.com/paholg/dimensioned.git"
git fetch upstream
git reset upstream/gh-pages

echo "dimensioned.paholg.com" > CNAME

touch .
git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
