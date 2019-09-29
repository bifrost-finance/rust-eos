#!/bin/bash
cleos='/usr/local/bin/cleos -u http://47.101.139.226:8888'

#DEFAULT_PASSWD_PATH="/default.passwd"
#$cleos wallet create --file $DEFAULT_PASSWD_PATH
#$cleos wallet open

./eos-wallet-unlock.sh

echo '5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3' | $cleos wallet import
echo '5HrPPFF2hq1X8ktBVfUVubeAmSaerRHwz2aGxGSUqvAuaNhR8a5' | $cleos wallet import
echo '5JXvFAgCsdDTgUXXqXk6QNmqHdMhPirvDuHrmyJSHLQS7bKj3bv' | $cleos wallet import
echo '5Jz2yvc5sqEUnwQhvJLAC9bNYiCdrxNi1WfGHpYf9bPgnsVhAAx' | $cleos wallet import

