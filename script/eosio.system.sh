#!/bin/bash
cleos='docker exec -i eosio /opt/eosio/bin/cleos --url http://127.0.0.1:7777 --wallet-url http://127.0.0.1:5555'

# set contract
$cleos set contract eosio /Users/edwin/liebi/github/eosio.contracts/eosio.system build/eosio.system.wasm build/eosio.system.abi -p eosio@active

# init contract
$cleos push action eosio init '[0, "4,EOS"]' -p eosio@active
