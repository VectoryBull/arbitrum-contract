#!/usr/bin/env bash
. ./scripts/.env

if [ "$ENV" = "test" ]; then
    # Commands to execute if $var equals "test"
    . ./scripts/.env.test
else
    # Commands to execute if $var does not equal "test"
    . ./scripts/.env.prod
fi

cast send --rpc-url $RPC_URL --private-key $PK \
$CONTRACT "init(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)" 0 10 10 0 10 10 10 2753 3233

cast send --rpc-url $RPC_URL --private-key $PK \
$CONTRACT "storeSensorData(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)" 1 1 1 1 1 1 1 3086

cast send --rpc-url $RPC_URL --private-key $PK \
$CONTRACT "storeSensorData(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)" 1 1 1 1 1 1 1 3

cast call --rpc-url $RPC_URL \
--private-key $PK \
$CONTRACT "printTemp(uint256)(uint256)" 0 

cast storage --rpc-url $RPC_URL $CONTRACT 0 

cast send --rpc-url $RPC_URL --private-key $PK \
$CONTRACT "terminateDataCollection()"

cast call --rpc-url $RPC_URL \
--private-key $PK \
$CONTRACT "printFinalState()(uint256)" 