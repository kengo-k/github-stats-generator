#!/bin/sh
rm -rf dist
yarn tsc
cp package.json yarn.lock .mdb .env dist
cp -r src/public dist
cd dist
yarn install
