# ledger
[![Build Status](https://travis-ci.org/d2ci8xc5/ledger.svg?branch=master)](https://travis-ci.org/d2ci8xc5/ledger) [![codecov](https://codecov.io/gh/d2ci8xc5/ledger/branch/master/graph/badge.svg)](https://codecov.io/gh/d2ci8xc5/ledger)
### brief
```
Write a simple, interactive double-entry ledger with the following features:

* ability to create transactions to move assets between accounts
* ability to list accounts and their balances
* ability to list all transactions

Requirements:

* all transactions must balance out to 0

Bonus:

* state backed by file system
* codebase is tested

Terminology:

* "transaction" - A transaction is a set of transfers between accounts that
  always balances to 0.
  For example:

  ---
  2018/08/10 Payment
    assets_checking  $500
    assets_savings   $500
    income_salary  - $1000
  ---

  ---
  [<date>] <description>
    <account> [-|+] [$]<amount>
    <account> [-|+] [$]<amount>
    [<account> [-|+] [$]<amount>]...
  --

Notes:

* "interactivity" is left open for interpretation. It could be an HTTP server, a
  REPL, a simple TCP server over a Unix domain socket, a web app, or anything
  else.
* the method of serializing data to the screen/disk is open for interpretation.
```

