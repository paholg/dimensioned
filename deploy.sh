#!/usr/bin/env bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

# git init
git config --global user.name "Paho Lurie-Gregg"
git config --global user.email "paho@paholg.com"

git clone --branch=gh-pages "https://$GH_TOKEN@github.com/paholg/dimensioned.git" pages/

mv target/doc pages/

cd pages

git add -A .
git commit -m "Add doc at ${rev}"
git push -q upstream HEAD:gh-pages
