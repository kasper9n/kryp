#!/usr/bin/env bash
echo "Creating users..."
mongo admin --host localhost -u root -p ${MONGO_INITDB_ROOT_PASSWORD} --eval "db = db.getSiblingDB('cryptrack'); db.createUser({user: '${DB_USERNAME}', pwd: '${DB_PASSWORD}',roles: [{role: 'readWrite', db: 'cryptrack'}]});"
echo "Users created."
