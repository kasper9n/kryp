FROM node:12.16-alpine
WORKDIR /usr/src
COPY package.json ./
RUN npm install
WORKDIR /usr/src/app
COPY . .
CMD ["npm", "run", "start-dev"]
