FROM node:12.16-alpine AS dev-stage
WORKDIR /usr/src/app
COPY package.json ./
RUN npm install
COPY . .
CMD ["npm", "run", "serve"]
