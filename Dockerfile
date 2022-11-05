FROM node:14.21.0-bullseye-slim

WORKDIR /home/node
COPY . /home/node

RUN \
  yarn; \
  yarn build;

ENTRYPOINT ["yarn", "start"]
