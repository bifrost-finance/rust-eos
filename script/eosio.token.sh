#!/bin/bash
cleos='/usr/local/bin/cleos -u http://47.101.139.226:8888'

# set contract
$cleos set contract eosio.token /Users/edwin/liebi/github/eosio.contracts/cmake-build-debug/contracts/eosio.token eosio.token.wasm eosio.token.abi -p eosio.token@active

# create token
$cleos push action eosio.token create '{"issuer":"eosio", "maximum_supply":"1000000000.0000 EOS"}' -p eosio.token@active
$cleos push action eosio.token issue '["eosio", "10000000.0000 EOS", "memo"]' -p eosio@active

# issue token
$cleos push action eosio.token transfer '["eosio", "alice", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "bob", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "lurpis", "100000.0000 EOS", "memo"]' -p eosio@active

$cleos push action eosio.token transfer '["eosio", "testa", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testb", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testc", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testd", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "teste", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testf", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testg", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testh", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testi", "100000.0000 EOS", "memo"]' -p eosio@active
$cleos push action eosio.token transfer '["eosio", "testj", "100000.0000 EOS", "memo"]' -p eosio@active
