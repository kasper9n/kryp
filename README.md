# Cryptrack

crypto data api alterantives
- https://www.coinapi.io/
- https://min-api.cryptocompare.com
- https://coinmarketcap.com/api/
- https://iexcloud.io/pricing/

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

API_SESSION_KEY=secret

PRODUCTION_API_URL=https://api.cryptrack.io

API_HTTP_PORT=80
API_HTTPS_PORT=443
WEB_HTTP_PORT=8080
WEB_HTTPS_PORT=8443
DB_PORT=27017
```

[lazydocker](https://github.com/jesseduffield/lazydocker) is a pretty nice CLI GUI for managing docker-compose.

### Structure
The project consists of 3 docker containers:
- `db`: MongoDB database
- `api`: Node.js backend
- `web`: Vue.js frontend

### Development environment

`web` has a dev server accessible at `WEB_HTTP_PORT` and `WEB_HTTPS_PORT`. This is served through `api` as a proxy as a workaround for Vue.js only serving at one port.

`web` and `api` serve both HTTP and HTTPS connections. For HTTPS, a self-signed SSL certificates is generated. Browsers will show warnings due to this, but you should be able to bypass thar.

You can test the HTTPS connections by pointing a domain to you using a proxy service like CloudFlare, and forwarding the relevant ports (Recommend [Port Map](https://www.codingmonkeys.de/portmap/) for macOS). Make sure that you only use ports that the proxy service supports.


### Commands
Start:
```
docker-compose up
```

Rebuild the Docker images (e.g for when Dockerfile or npm dependencies change):
```
docker-compose build
```

Lint frontend code:
```
make lint
```

Lint and fix frontend code:
```
make lint-fix
```

### Connecting to the database
To connect to the database, for example via a GUI app like MongoDB Compass, use the following connection string:
```
mongodb://DB_USERNAME:DB_PASSWORD>@HOST:DB_PORT/admin?authSource=admin
```
`DB_USERNAME`, `DB_PASSWORD` and `DB_PORT` correspond to the values in your `.env` file. `HOST` is `localhost` for development, and your server IP for production (You could use a domain name as well, but proxies like CloudFlare only support a few ports).

### Deployment
*Unfinished section*

#### Initial setup
1. Set up a server, for example a DigitalOcean droplet. `api` and `db` will be served from here
2. `api` uses a self-signed SSL certificate. Use a proxy service like CloudFlare to provide a real SSL certificate.
3. Deploy `web` to a static site host like Netlify.
4. Set up domains for `web` and `api`. If your `api` domain is `example.com`, then set `PRODUCTION_API_URL` to `https://example.com` inside `.env`.

### TODO
- Email confirmation, probably using something like nodemailer
- Use custom, shorter IDs in database
