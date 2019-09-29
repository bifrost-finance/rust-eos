#!/bin/bash
docker stop eosio > /dev/null
docker rm eosio > /dev/null
docker run --name eosio \
  --publish 7777:7777 \
  --publish 5555:5555 \
  --volume /Users/edwin:/Users/edwin \
  --detach \
  eosio/eos:v1.5.1 \
  /bin/bash -c \
  "keosd --plugin eosio::wallet_api_plugin --http-alias=localhost:5555 --http-server-address=0.0.0.0:5555 & exec nodeos -e -p eosio --max-transaction-time=1000 --plugin eosio::producer_plugin --plugin eosio::chain_api_plugin --plugin eosio::history_plugin --plugin eosio::history_api_plugin --plugin eosio::http_plugin -d /mnt/dev/data --config-dir /mnt/dev/config --http-server-address=0.0.0.0:7777 --access-control-allow-origin=* --contracts-console --http-validate-host=false --filter-on='*' --verbose-http-errors --p2p-peer-address=bp.eosbeijing.one:8080" > /dev/null

#docker ps -a
./eos-init.sh
./eos-create-account.sh
./eosio.token.sh
./eosio.system.sh
./boomgo.token.sh
./boomgo.cf.sh
./liebi.stake.sh
./liebi.bank.sh
