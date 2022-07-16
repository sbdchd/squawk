# linter/src/rules

## Adding a new rule (automated)

1. Install [plop](https://github.com/plopjs/plop)
   ```
   npm install -g plop
   ```
2. Generate a new rule
   ```
   s/new-rule 'my-rule-name'
   ```

## Adding a new rule (manually)

1. Create a new file in `squawk/linter/src/rules/my_new_rule.rs`. Use an existing rule as a template.
2. Add your module to `squawk/linter/src/rules/mod.rs`.
   ```rust
   pub mod my_new_rule;
   pub use my_new_rule::*;
   ```
3. Add your linter to the `RULES` list in `squawk/linter/lib.rs`.
4. Document your rule in `squawk/docs/docs/my-new-rule.md`.
5. Add your docs page ID to `squawk/docs/sidebars.js`.
6. Add your docs page ID to `squawk/docs/src/pages/index.js`.
