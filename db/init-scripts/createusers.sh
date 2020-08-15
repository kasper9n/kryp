#!/usr/bin/env bash
echo "Creating users..."
mongo admin --host localhost -u ${MONGO_INITDB_ROOT_USERNAME} -p ${MONGO_INITDB_ROOT_PASSWORD} --eval "db = db.getSiblingDB('kryp'); db.createUser({user: '${DB_USERNAME}', pwd: '${DB_PASSWORD}',roles: [{role: 'readWrite', db: 'kryp'}]});"
echo "Users created."
