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

Lint frontend code:
```
make lint
```

Lint and fix frontend code:
```
make lint-fix
```

### Connecting to the database
To connect to the database, for example via a GUI app like MongoDB Compass, you can use the following connection string:
```
mongodb://<DB_USERNAME>:<DB_PASSWORD>@<HOST>:27017/admin
```
- `DB_USERNAME`: The value you wrote in your `.env` file
- `DB_PASSWORD`: The value you wrote in your `.env` file
- `HOST`: `localhost` for local development, your server IP for production
Replace DB_USERNAME and DB_PASSWORD with the corresponding environment variables in your `.env` file. For local development, replace `HOST` with `localhost`.



### Deployment
TBA. Will probably use Docker Contexts

### TODO
- Email confirmation, probably using something like nodemailer
- Use custom, shorter IDs in database
