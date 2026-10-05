import React from "react"
import clsx from "clsx"
import Layout from "@theme/Layout"
import Link from "@docusaurus/Link"
import useDocusaurusContext from "@docusaurus/useDocusaurusContext"
import useBaseUrl from "@docusaurus/useBaseUrl"
import styles from "./styles.module.css"

const features = [
  {
    title: "Prevent Downtime",
    description: (
      <>Lint your schema changes and prevent blocking reads / writes.</>
    ),
  },
  {
    title: "GitHub Integration",
    description: (
      <>
        Use the <Link to={"/docs/github_app"}>Squawk GitHub App</Link> to lint
        your pull requests.
      </>
    ),
  },
  {
    title: "VSCode Support",
    description: (
      <>
        Squawk's language server and <a href="https://marketplace.visualstudio.com/items?itemName=squawk.squawk-vscode">VSCode extension</a> provides linting in your editor.
      </>
    ),
  },
]

function Feature({ imageUrl, title, description }) {
  const imgUrl = useBaseUrl(imageUrl)
  return (
    <div className={clsx("col col--4", styles.feature)}>
      {imgUrl && (
        <div className="text--center">
          <img className={styles.featureImage} src={imgUrl} alt={title} />
        </div>
      )}
      <h3>{title}</h3>
      <p>{description}</p>
    </div>
  )
}

const rules = [
  {
    name: "adding-field-with-default",
    tags: ["locking"],
    description:
      "Prevent blocking reads/writes to table while table is rewritten on PG < 11.",
  },
  {
    name: "adding-foreign-key-constraint",
    tags: ["locking"],
    description:
      "Prevent blocking writes to tables while verifying foreign key constraint.",
  },
  {
    name: "adding-not-nullable-field",
    tags: ["locking"],
    description:
      "Prevent blocking reads/writes to table while table is scanned on PG < 11.",
  },
  {
    name: "adding-required-field",
    tags: ["backwards compatibility"],
    description: "Prevent adding a new required field to an existing table.",
  },
  {
    name: "adding-serial-primary-key-field",
    tags: ["locking"],
    description: "Prevent blocking reads/writes to table while index is built.",
  },
  {
    name: "ban-char-field",
    tags: ["schema"],
    description: "Prevent mistaken use of character type in schema.",
  },
  {
    name: "ban-drop-database",
    tags: ["backwards compatibility"],
    description:
      "Prevent breaking existing clients that depend on the database.",
  },
  {
    name: "ban-drop-not-null",
    tags: ["backwards compatibility"],
    description: "Prevent breaking existing clients that don't expect NULL values.",
  },
  {
    name: "ban-drop-table",
    tags: ["backwards compatibility"],
    description:
        "Prevent breaking existing clients that depend on the table.",
  },
  {
    name: "changing-column-type",
    tags: ["backwards compatibility", "locking"],
    description:
      "Prevent breaking existing clients that depend on column type. Prevent blocking reads/writes to table while table is rewritten.",
  },
  {
    name: "constraint-missing-not-valid",
    tags: ["locking"],
    description: "Prevent blocking writes to the table while the scan occurs.",
  },
  {
    name: "disallowed-unique-constraint",
    tags: ["locking"],
    description: "Prevent blocking reads/writes to table while index is built.",
  },
  {
    name: "prefer-bigint-over-int",
    tags: ["schema"],
    description: "Prevent hitting the 32 bit max int limit.",
  },
  {
    name: "prefer-bigint-over-smallint",
    tags: ["schema"],
    description: "Prevent hitting the 16 bit max int limit.",
  },
  {
    name: "prefer-identity",
    tags: ["schema"],
    description: "Serial types have confusing behaviors. Use identity columns instead.",
  },
  {
    name: "prefer-robust-stmts",
    tags: ["locking"],
    description: "Ensure migrations are atomic or retriable.",
  },
  {
    name: "prefer-text-field",
    tags: ["locking"],
    description:
      "Prevent blocking reads and writes to table while table metadata is updated.",
  },
  {
    name: "prefer-timestamptz",
    tags: ["schema"],
    description:
      "Ensure consistent timezone handling for timestamps, regardless of your database session timezone.",
  },
  {
    name: "renaming-column",
    tags: ["backwards compatibility"],
    description: "Prevent breaking existing clients that depend on column.",
  },
  {
    name: "renaming-table",
    tags: ["backwards compatibility"],
    description: "Prevent breaking existing clients that depend on table.",
  },
  {
    name: "require-concurrent-index-creation",
    tags: ["locking"],
    description: "Prevent blocking writes to table while index is created.",
  },
  {
    name: "require-concurrent-index-deletion",
    tags: ["locking"],
    description:
      "Prevent blocking reads/writes to table while index is dropped.",
  },
  {
    name: "transaction-nesting",
    tags: ["locking"],
    description: "Ensure migrations use transactions correctly.",
  },
  {
    name: "ban-concurrent-index-creation-in-transaction",
    tags: ["schema"],
    description: "Prevent forbidden use of transactions during concurrent index creation.",
  },
  {
    name: "ban-create-domain-with-constraint",
    tags: ["schema", "locking"],
    description: "Domains with constraints have poor support for online migrations",
  },
  {
    name: "ban-alter-domain-with-add-constraint",
    tags: ["schema", "locking"],
    description: "Domains with constraints have poor support for online migrations",
  },
  {
    name: "ban-truncate-cascade",
    tags: ["backwards compatibility"],
    description: "Truncate cascade will recursively truncate all related tables!",
  },
  {
    name: "require-lock-timeout",
    tags: ["locking"],
    description: "Require a lock timeout",
  },
  {
    name: "require-statement-timeout",
    tags: ["locking"],
    description: "Require a statement timeout",
  },
  {
    name: "ban-uncommitted-transaction",
    tags: ["schema"],
    description: "Ensure all transactions are committed",
  },
  {
    name: "require-enum-value-ordering",
    tags: ["schema"],
    description: "Require BEFORE or AFTER when adding enum values",
  },
  {
    name: "require-table-schema",
    tags: ["schema"],
    description: "Require explicit schema in table DDL to avoid ambiguity.",
  },
  {
    name: "identifier-too-long",
    tags: ["schema"],
    description: "Prevent implicit truncation for identifiers that are too long.",
  },
  {
    name: "require-concurrent-partition-detach",
    tags: ["schema", "locking"],
    description: "Prevent blocking reads/writes to table while partition is detached.",
  },
  {
    name: "require-concurrent-reindex",
    tags: ["locking"],
    description: "Prevent blocking reads/writes to table while index is reindexed.",
  },
  {
    name: "prefer-repack",
    tags: ["locking"],
    description: "Prevent blocking reads/writes to table when rebuilding.",
  },
  {
    name: "ban-duplicate-column-assignments",
    tags: ["queries"],
    description: "Prevent invalid assignments to the same column more than once.",
  },
  {
    name: "ban-drop-view",
    tags: ["backwards compatibility"],
    description: "Prevent breaking clients that depend on views or materialized views.",
  },
  {
    name: "ban-drop-function",
    tags: ["backwards compatibility"],
    description: "Prevent breaking clients that call functions or procedures.",
  },
  {
    name: "ban-drop-type",
    tags: ["backwards compatibility"],
    description: "Prevent breaking clients that use dropped types.",
  },
  {
    name: "ban-drop-default",
    tags: ["backwards compatibility"],
    description: "Prevent inserts from failing or writing NULL after dropping a column default.",
  },
  {
    name: "ban-drop-trigger",
    tags: ["backwards compatibility"],
    description: "Prevent silent changes when a trigger is dropped (opt-in).",
  },
  { name: "ban-drop-schema", tags: ["backwards compatibility"], description: "Prevent breaking clients that use a dropped schema." },
  { name: "ban-drop-sequence", tags: ["backwards compatibility"], description: "Prevent breaking clients that use a dropped sequence." },
  { name: "ban-drop-domain", tags: ["backwards compatibility"], description: "Prevent breaking clients that use a dropped domain." },
  { name: "ban-drop-constraint", tags: ["backwards compatibility"], description: "Prevent removing a constraint guarantee." },
  { name: "ban-drop-generated-expression", tags: ["backwards compatibility"], description: "Prevent dropping a generated expression." },
  { name: "renaming-object", tags: ["backwards compatibility"], description: "Prevent renaming objects used by clients." },
  { name: "ban-set-schema", tags: ["backwards compatibility"], description: "Prevent moving objects used by clients to another schema." },
  { name: "ban-alter-identity", tags: ["backwards compatibility"], description: "Prevent changing identity columns used by clients." },
  { name: "ban-alter-generated-expression", tags: ["backwards compatibility"], description: "Prevent breaking inserts with generated columns (opt-in)." },
  { name: "ban-drop-index", tags: ["backwards compatibility"], description: "Prevent dropping indexes used by clients (opt-in)." },
  { name: "ban-set-default", tags: ["backwards compatibility"], description: "Prevent silent changes to column defaults (opt-in)." },
  { name: "ban-disable-trigger", tags: ["backwards compatibility"], description: "Prevent changes to triggers, rules, and row level security (opt-in)." },
  { name: "ban-replica-identity", tags: ["backwards compatibility"], description: "Prevent changes to replica identity (opt-in)." },
  { name: "ban-drop-policy", tags: ["backwards compatibility"], description: "Prevent dropping policies and rules (opt-in)." },
  { name: "ban-revoke", tags: ["backwards compatibility"], description: "Prevent revoking client privileges (opt-in)." },
  { name: "ban-replace-view-function", tags: ["backwards compatibility"], description: "Prevent replacing views and routines (opt-in)." },
  { name: "ban-drop-extension", tags: ["backwards compatibility"], description: "Prevent dropping extensions used by clients." },
  { name: "ban-create-policy", tags: ["backwards compatibility"], description: "Review new policy access rules (opt-in)." },
  { name: "ban-alter-policy-condition", tags: ["backwards compatibility"], description: "Review policy condition changes (opt-in)." },
  { name: "ban-alter-policy-roles", tags: ["backwards compatibility"], description: "Review policy role changes (opt-in)." },
  { name: "ban-alter-function-options", tags: ["backwards compatibility"], description: "Review function option changes (opt-in)." },
  { name: "ban-alter-view-options", tags: ["backwards compatibility"], description: "Review view option changes (opt-in)." },
  { name: "ban-alter-role-options", tags: ["backwards compatibility"], description: "Review role option and configuration changes (opt-in)." },
  { name: "ban-alter-database-options", tags: ["backwards compatibility"], description: "Review database option and configuration changes (opt-in)." },
  { name: "ban-alter-row-level-security", tags: ["backwards compatibility"], description: "Review row level security changes (opt-in)." },
  { name: "ban-new-write-restriction", tags: ["backwards compatibility"], description: "Detect new write restrictions on existing tables (opt-in)." },
  { name: "ban-add-enum-value", tags: ["backwards compatibility"], description: "Detect new enum values (opt-in)." },
  { name: "ban-add-composite-attribute", tags: ["backwards compatibility"], description: "Detect new composite attributes (opt-in)." },
  { name: "ban-detach-inheritance", tags: ["backwards compatibility"], description: "Detect partition detach and NO INHERIT (opt-in)." },
  { name: "ban-alter-sequence-values", tags: ["backwards compatibility"], description: "Detect changes to sequence values (opt-in)." },
  // xtask:new-rule:rule-doc-meta
]

function Home() {
  const context = useDocusaurusContext()
  const { siteConfig = {} } = context
  return (
    <Layout>
      <header className={clsx("hero hero--primary", styles.heroBanner)}>
        <div className="container">
          <h1 className="hero__title">Squawk</h1>
          <p className="hero__subtitle">
            A linter and language server for Postgres migrations & SQL
          </p>
          <div className={styles.buttons} style={{display: 'flex', flexDirection: 'column', gap: '1rem', alignItems: 'center'}}>
            <div style={{display: 'flex', alignItems: 'center', gap: "0.5rem"}}>
              <code style={{fontSize: '1rem', margin: 0, color: '#333', fontFamily: 'monospace', padding: "0.5rem 1rem", flexGrow: 1}}>
                npm install -g squawk-cli
              </code>
            </div>
            <Link
              className={clsx(
                "button button--secondary button--lg",
                styles.getStarted
              )}
              style={{minWidth: "400px"}}
              to="https://marketplace.visualstudio.com/items?itemName=sbdchd.squawk">
              Install VSCode Extension
            </Link>

            <Link to={useBaseUrl("docs/")} style={{color: "white"}}>See other install methods</Link>
          </div>
        </div>
      </header>
      <main>
        {features && features.length > 0 && (
          <section className={styles.features}>
            <div className="container">
              <div className="row" style={{paddingBottom: '2rem'}}>
                {features.map((props, idx) => (
                  <Feature key={idx} {...props} />
                ))}
              </div>
              <div className="row" />
              <div className="row">
                <div className="col">
                  <a href="/docs/rules">
                    <h3 style={{ color: "var(--ifm-font-color-base)" }}>
                      Rules
                    </h3>
                  </a>
                  {[
                    { title: "Prevent schema mistakes", tags: ["schema"] },
                    {
                      title: "Make backwards compatible schema changes",
                      tags: ["backwards compatibility"],
                    },
                    { title: "Apply schema changes safely", tags: ["locking"] },
                  ].map((sec) => (
                    <>
                      <h4 style={{marginBottom: '0.5rem'}}>{sec.title}</h4>
                      <table style={{marginBottom: '2rem'}}>
                        <tr>
                          <th>rule name</th>
                          <th>description</th>
                        </tr>
                        {rules
                          .filter((rule) =>
                            sec.tags.some((tag) => rule.tags.includes(tag))
                          )
                          .map((rule) => (
                            <tr key={rule.name}>
                              <td style={{ wordBreak: "keep-all" }}>
                                <a href={"/docs/" + rule.name}>{rule.name}</a>
                              </td>
                              <td>{rule.description}</td>
                            </tr>
                          ))}
                      </table>
                    </>
                  ))}
                </div>
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  )
}

export default Home
