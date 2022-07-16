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
          template: `
pub mod {{RuleNameSnake}};
pub use {{RuleNameSnake}}::*;
          `,
        },        
        {
          type: 'modify',
          path: 'linter/src/violations.rs',
          template: `
    #[serde(rename = "{{RuleNameKebab}}")]
    {{RuleNameCamel}},
          `,
        },
        {
          type: 'modify',
          path: 'linter/src/lib.rs',
          template: `
    SquawkRule {
        name: RuleViolationKind::{{RuleNameCamel}},
        func: {{RuleNameSnake}},
        messages: vec![
            ViolationMessage::Note(
                "TODO".into()
            ),
            ViolationMessage::Help(
                "TODO".into()
            ),
        ],
    }
`,
        },
        {
          type: 'modify',
          path: 'docs/sidebar.js',
          template: `"{{RuleNameKebab}}"`,
        },
      ]
    },
  })
}