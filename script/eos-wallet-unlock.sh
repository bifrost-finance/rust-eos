#!/bin/bash
cleos='/usr/local/bin/cleos -u http://47.101.139.226:8888'

DEFAULT_PASSWD_PATH="/Users/edwin/eosio-wallet/password"
cat $DEFAULT_PASSWD_PATH | $cleos wallet unlock
cat $DEFAULT_PASSWD_PATH | $cleos wallet private_keys

#docker exec -it eosio cat $BANK_PASSWD_PATH | $cleos wallet unlock -n bank
#docker exec -it eosio cat $BANK_PASSWD_PATH | $cleos wallet private_keys -n bank

