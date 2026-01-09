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
    name: "require-timeout-settings",
    tags: ["locking"],
    description: "Require lock and statement timeouts",
  },
  {
    name: "ban-uncommitted-transaction",
    tags: ["schema"],
    description: "Ensure all transactions are committed",
  },
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
