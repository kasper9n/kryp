FROM node:14.11-alpine AS dev-stage
WORKDIR /usr/src/app
COPY package.json ./
COPY . .
CMD ["sh", "-c", "yarn install --no-progress && yarn run dev"]

FROM node:14.11-alpine AS build-stage
WORKDIR /usr/src/app
COPY package.json ./
RUN yarn install
COPY . .
RUN yarn run build

FROM node:14.11-alpine AS production-stage
COPY --from=build-stage /usr/src/app /usr/src/app
WORKDIR /usr/src/app
CMD ["yarn", "run", "start"]
