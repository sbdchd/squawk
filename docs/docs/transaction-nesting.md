---
id: transaction-nesting
title: transaction-nesting
---

## problem

PostgreSQL doesn't support transactions that are nested inside of each other. There is only one transaction running per session. The server will warn if `BEGIN`, `START TRANSACTION`, `COMMIT`, `ROLLBACK`, or `END` statements are repeated.

If you use the `--assume-in-transaction` flag when running Squawk or set `assume-in-transaction = true` in your config file, Squawk assumes that your migration tool will wrap each migration file in a transaction at runtime. Therefore, explicit calls to `BEGIN` and `COMMIT` should be avoided, as these may disrupt what the tool is doing and cause unpredictable results.

## solution

Fix by removing the offending statements, verifying that the transaction start and end are being set correctly.

If you are using `assume-in-transaction`, split each transaction into its own file.

Alternatively, if your migration tool supports explicitly managing transactions, you could use that along with `BEGIN` and `COMMIT` statements in your migration file and not set `assume-in-transaction` for Squawk.
