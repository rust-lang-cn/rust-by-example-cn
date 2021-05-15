#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd book

git init
git config user.name "Aaran Xu"
git config user.email "aaranxu@outlook.com"
git remote add upstream "git@github.com:rust-lang-cn/rust-by-example-cn.git"
git fetch upstream && git reset upstream/gh-pages

# echo "rustwiki.org" > CNAME

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
