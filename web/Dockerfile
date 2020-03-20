FROM node:12.16-alpine AS dev-stage
WORKDIR /usr/src
COPY package.json ./
RUN npm install
WORKDIR /usr/src/app
COPY . .
CMD ["npm", "run", "serve"]
