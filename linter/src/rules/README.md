# linter/src/rules

## Adding a new rule

1. Create a new file in `squawk/linter/src/rules/my_new_rule.rs`. Use an existing rule as a template.
2. Add your module to `squawk/linter/src/rules/mod.rs`.
   ```rust
   pub mod my_new_rule;
   pub use my_new_rule::*;
   ```
3. Add your linter to the `RULES` list in `squawk/linter/lib.rs`.
4. Document your rule in `squawk/docs/docs/my-new-rule.md`.
5. Add your docs page ID to `squawk/docs/sidebars.js`.
