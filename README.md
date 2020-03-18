# Cryptrack

crypto data api alterantives
- https://www.coinapi.io/
- https://min-api.cryptocompare.com
- https://coinmarketcap.com/api/

## Dev instructions

### Getting started
1. Install [Docker](https://docs.docker.com/install/)
2. If Docker didn't come with [Docker Compose](https://docs.docker.com/compose/install/), install that too
3. Create a `.env` file that looks like this:
```
# when updating, remember to update README.md
DB_PASSWORD=my_password
DB_ROOT_PASSWORD=my_password
```

### Commands
Start:
```
docker-compose up
```

Rebuild the Docker images (e.g for when Dockerfile or npm dependencies change):
```
docker-compose build
```

### Deployment
TBA. Will probably use Docker Contexts
