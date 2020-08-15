# Kryp

## Dev instructions

### Getting started
1. Install [Docker](https://docs.docker.com/install/)
2. If Docker didn't come with [Docker Compose](https://docs.docker.com/compose/install/), install that too
3. Create a `.env` file that looks like this:
```dotenv
# when updating, remember to update README.md
# DO NOT USE SPECIAL CHARS
DB_ROOT_USERNAME=root
DB_ROOT_PASSWORD=secret
DB_USERNAME=appuser
DB_PASSWORD=secret
DB_PORT=27017

SERVER_PORT=80
```

[lazydocker](https://github.com/jesseduffield/lazydocker) is a pretty nice CLI GUI for managing docker-compose.

### Commands

Start:
```
docker-compose up
```

Rebuild the Docker images (e.g for when Dockerfile or npm dependencies change):
```
docker-compose build
```

Open server shell. For instance, this lets you run `go get` and `go mod` to manage dependencies - Just remember to run `docker-compose build` afterwards:
```
make server-sh
```

### Connecting to the database
To connect to the database, for example via a GUI app like MongoDB Compass, use the following connection string:
```
mongodb://DB_ROOT_USERNAME:DB_ROOT_PASSWORD>@HOST:DB_PORT/admin
```
`DB_ROOT_USERNAME`, `DB_ROOT_PASSWORD` and `DB_PORT` correspond to the values in your `.env` file. `HOST` is `localhost` for development, and your server IP for production (If you use a domain name, proxies like CloudFlare may block the port).
