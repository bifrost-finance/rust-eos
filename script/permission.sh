cleos-ark set account permission testa active '{"threshold":2,"keys":[],"accounts":[{"permission":{"actor":"alice","permission":"active"},"weight":1}, {"permission":{"actor":"bob","permission":"active"},"weight":1}]}' owner
cleos-ark set account permission testa owner '{"threshold":2,"keys":[],"accounts":[{"permission":{"actor":"alice","permission":"active"},"weight":1}, {"permission":{"actor":"bob","permission":"active"},"weight":1}]}' -p testa@owner

cleos-ark get account testa
# 生成交易报文
cleos-ark transfer testa testb "1 EOS" "" -d -s -x 300
cleos-ark transfer testa testb "1 EOS" "" -d -s -x 300 --return-packed > tx.json 2>&1
cleos-ark transfer testa testb "1 EOS" "" -f -s -d -x 300 > tx.json 2>&1
cleos-ark sign tx.json -k 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
cleos-ark sign tx.json -k 5HrPPFF2hq1X8ktBVfUVubeAmSaerRHwz2aGxGSUqvAuaNhR8a5
cleos-ark push transaction tx-sign.json
cleos-ark push transaction tx.json

cleos-ark get currency balance eosio.token testb eos

password: [[
    "EOS6ErjVHSouF2qMfNHcc2m9eiwguUQ2LGgwJSWoZmBhLdqpYtdMx",
    "5JXvFAgCsdDTgUXXqXk6QNmqHdMhPirvDuHrmyJSHLQS7bKj3bv"
  ],[
    "EOS6JzuvvcooELoDpCJKdUSTVR8gpaPTNXNBsDccjc2RitDGxr35h",
    "5Jz2yvc5sqEUnwQhvJLAC9bNYiCdrxNi1WfGHpYf9bPgnsVhAAx"
  ],[
    "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV",
    "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3"
  ],[
    "EOS7tdB5cKCjhjZqyMNY9deCKD6CsHnxbdY9XE8k5vgAt3gLJwQWh",
    "5HrPPFF2hq1X8ktBVfUVubeAmSaerRHwz2aGxGSUqvAuaNhR8a5"
  ]
]

cleos-ark transfer testa testb "1 EOS" "" -d -s -x 300
cleos-ark transfer testa testb "1 EOS" "" -d -s -x 300 --return-packed
