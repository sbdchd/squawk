---
id: rules
title: Rules Overview
---

Squawk's rules focus on ensuring safe migrations and warn about statements that could block reads / writes or break existing clients. See the sidebar for documentation for each rule.

The rule docs should provide actionable, user friendly error messages. If you find a rule, error message, or docs page lacking, please [open an issue](https://github.com/sbdchd/squawk/issues/new).


:::note Reminder

Read ["Running Migrations](./safe_migrations.md) to learn about safely applying migrations after linting.
:::

## Index
import sidebar from '../sidebars.js'

<ul>
{sidebar.someSidebar.Rules.map(ruleName => <li key={ruleName}><a href={`/docs/${ruleName}`}>{ruleName}</a></li>)}
</ul>
