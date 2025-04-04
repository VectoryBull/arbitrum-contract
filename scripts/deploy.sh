#!/usr/bin/env bash
. ./scripts/.env

if [ "$ENV" = "test" ]; then
    # Commands to execute if $var equals "test"
    . ./scripts/.env.test
else
    # Commands to execute if $var does not equal "test"
    . ./scripts/.env.prod
fi

cargo stylus deploy \
  --endpoint=$RPC_URL \
  --private-key=$PK

cargo stylus export-abi > abi.json