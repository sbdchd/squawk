// configuration for plop to generate a new Squawk rule.
// use `s/new-rule` to call this. 
// You must install `plop` via `npm install -g plop`.
//
// https://github.com/plopjs/plop
module.exports = function (plop) {
   plop.setGenerator('rule', {
    description: 'Create a new rule',
    prompts: [
      {
        name: 'name',
        type: 'input',
        message: 'Name for the rule',
      },
    ],
    actions: function (data) {
      data.RuleNameKebab = plop.getHelper("kebabCase")(data.name);
      data.RuleNameSnake = plop.getHelper("snakeCase")(data.name);
      data.RuleNameCamel = plop.getHelper("camelCase")(data.name);
      data.RuleNamePascal = plop.getHelper("pascalCase")(data.name);
      return [
        {
          type: 'add',
          path: `linter/src/rules/{{RuleNameSnake}}.rs`,
          templateFile: `templates/new_rule.rs.template`,
        },
        {
          type: 'add',
          path: `docs/docs/{{RuleNameSnake}}.md`,
          templateFile: `templates/new_rule.md.template`,
        },  
        {
          type: 'modify',
          path: 'linter/src/rules/mod.rs',
          pattern: /$/,
          template: 
`pub mod {{RuleNameSnake}};
pub use {{RuleNameSnake}}::*;`,
        },        
        {
          type: 'modify',
          path: 'linter/src/violations.rs',
          pattern: /\/\/\sgenerator::new-rule-above/,
          template: 
`#[serde(rename = "{{RuleNameKebab}}")]
    {{RuleNamePascal}},
    // generator::new-rule-above`,
        },
        {
          type: 'modify',
          path: 'linter/src/lib.rs',
          pattern: /\/\/\sgenerator::new-rule-above/,
          template: 
`SquawkRule {
        name: RuleViolationKind::{{RuleNamePascal}},
        func: {{RuleNameSnake}},
        messages: vec![
            ViolationMessage::Note(
                "TODO".into()
            ),
            ViolationMessage::Help(
                "TODO".into()
            ),
        ],
    },
    // generator::new-rule-above`,
        },
        {
          type: 'modify',
          path: 'linter/src/lib.rs',
          pattern: /use crate::rules/,
          template: 
`use crate::rules::{{RuleNameSnake}};
use crate::rules`,
        },
        {
          type: 'modify',
          path: 'docs/sidebars.js',
          pattern: /\/\/\sgenerator::new-rule-above/,
          template: `"{{RuleNameKebab}}",
      // generator::new-rule-above`,
        },
        {
          type: 'modify',
          path: 'docs/src/pages/index.js',
          pattern: /\/\/\sgenerator::new-rule-above/,
          template: 
`{
    name: "{{RuleNameKebab}}",
    tags: ["TODO"],
    description: "TODO",
  },
  // generator::new-rule-above`,
        },
      ]
    },
  })
}