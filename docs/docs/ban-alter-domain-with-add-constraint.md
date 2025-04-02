---
id: ban-alter-domain-with-add-constraint
title: ban-alter-domain-with-add-constraint
---

## problem

Postgres [domains][], which associate a data type with an optional check constraint, have poor support for online migrations
when associated with a check constraint.

[domains]: https://www.postgresql.org/docs/current/sql-createdomain.html

The purpose of domains is to make the named type-plus-constraint reusable, but this means that any change to the domain's constraint
requires _all_ columns that use the domain to be revalidated. And, because Postgres can't reason well about arbitrary constraints,
they increase the chances of a change requiring an expensive table rewrite.

A couple relevant quotes from a Postgres developer include:

> No, that's not going to work: coercing to a domain that has any
> constraints is considered to require a rewrite.

And:

> In any case, the point remains that domains are pretty inefficient
> compared to native types like varchar(12); partly because the system
> can’t reason very well about arbitrary check constraints as compared
> to simple length constraints, and partly because the whole feature
> just isn’t implemented very completely or efficiently.  So you’ll be
> paying *a lot* for some hypothetical future savings.


## solution

Either avoid domains altogether, or (most importantly) avoid adding constraints to domains. Instead, put the [constraint][]
on the desired column(s) directly.

[constraint]: https://www.postgresql.org/docs/current/sql-createdomain.html

## links

[The mailing list thread from which the above quotes are sourced](https://www.postgresql.org/message-id/flat/CADVWZZKjhV9fLpewPdQMZx7V6kvGJViwMEDrPAv9m50rGeK9UA%40mail.gmail.com)
