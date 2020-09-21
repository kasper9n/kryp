FROM node:14.11-alpine AS dev-stage
WORKDIR /app
COPY package.json ./
COPY . .
CMD ["sh", "-c", "yarn install --no-progress && yarn run dev"]

FROM node:14.11-alpine AS build-stage
WORKDIR /app
COPY package.json ./
RUN yarn install
COPY . .
RUN yarn run build

FROM node:14.11-alpine AS production-stage
COPY --from=build-stage /app /app
WORKDIR /app
CMD ["yarn", "run", "start"]
