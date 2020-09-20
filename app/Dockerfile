FROM node:14.11-alpine AS dev-stage
WORKDIR /usr/src/app
COPY package.json ./
RUN npm install
COPY . .
CMD ["npm", "run", "dev"]

FROM node:14.11-alpine AS build-stage
COPY --from=dev-stage /usr/src/app /usr/src/app
WORKDIR /usr/src/app
RUN npm run build

FROM node:14.11-alpine AS production-stage
COPY --from=build-stage /usr/src/app /usr/src/app
WORKDIR /usr/src/app
CMD ["npm", "run", "start"]
