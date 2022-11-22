#!/bin/sh

#
# build backend
#

# move backend directory and init
cd backend
yarn install

# clean dist directory
rm -rf dist

# build
yarn tsc
cp package.json yarn.lock .env ../.mdb dist

#
# build frontend
#
cd ../frontend
yarn install
yarn build

cd ../backend
cp -r src/public dist
cd dist
yarn install
