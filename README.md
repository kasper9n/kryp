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
