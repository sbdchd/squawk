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
        Use the <Link to={"/docs/github_app"}>Squawk GitHub App</Link> to lint your pull requests.
      </>
    ),
  },
  {
    title: "Open Source",
    description: (
      <>
        <code>squawk</code> is open source and written in Rust. Install it with{" "}
        <code>npm install squawk-cli</code>.
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

function Home() {
  const context = useDocusaurusContext()
  const { siteConfig = {} } = context
  return (
    <Layout>
      <header className={clsx("hero hero--primary", styles.heroBanner)}>
        <div className="container">
          <h1 className="hero__title">Squawk</h1>
          <p className="hero__subtitle">A linter for Postgres migrations</p>
          <div className={styles.buttons}>
            <Link
              className={clsx(
                "button button--secondary button--lg",
                styles.getStarted
              )}
              to={useBaseUrl("docs/")}>
              Get Started
            </Link>
          </div>
        </div>
      </header>
      <main>
        {features && features.length > 0 && (
          <section className={styles.features}>
            <div className="container">
              <div className="row">
                {features.map((props, idx) => (
                  <Feature key={idx} {...props} />
                ))}
              </div>
              <div className="row">
                <div class="col">
                  <a href="/docs/rules">
                    <h3>Rules</h3>
                  </a>
                  <table>
                    <tr>
                      <th>name</th>
                      <th>kind</th>
                      <th>description</th>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/adding-field-with-default">
                          adding-field-with-default
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        {
                          "Prevent blocking reads/writes to table while table is rewritten on PG < 11."
                        }
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/adding-foreign-key-constraint">
                          adding-foreign-key-constraint
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking writes to tables while verifying
                        foreign key constraint.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/adding-not-nullable-field">
                          adding-not-nullable-field
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        {
                          "Prevent blocking reads/writes to table while table is scanned on PG < 11."
                        }
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/adding-serial-primary-key-field">
                          adding-serial-primary-key-field
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking reads/writes to table while index is
                        built.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/ban-char-field ">ban-char-field</a>
                      </td>
                      <td>schema</td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        Prevent mistaken use of character type in schema.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/ban-drop-database">ban-drop-database</a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        backwards compatability
                      </td>
                      <td>
                        Prevent breaking existing clients that depend on the
                        database.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/changing-column-type">
                          changing-column-type
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        backwards compatability, locking
                      </td>
                      <td>
                        Prevent breaking existing clients that depend on column
                        type. Prevent blocking reads/writes to table while table
                        is rewritten.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/constraint-missing-not-valid">
                          constraint-missing-not-valid
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking writes to the table while the scan
                        occurs.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/disallowed-unique-constraint">
                          disallowed-unique-constraint
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking reads/writes to table while index is
                        built.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/prefer-robust-stmts migrations safety">
                          prefer-robust-stmts
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        migrations safety
                      </td>
                      <td>Ensure migrations are atomic or retriable.</td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/prefer-text-field">prefer-text-field</a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking reads and writes to table while table
                        metadata is updated.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/renaming-column">renaming-column</a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        backwards compatability
                      </td>
                      <td>
                        Prevent breaking existing clients that depend on column.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/renaming-table">renaming-table</a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>
                        backwards compatability
                      </td>
                      <td>
                        Prevent breaking existing clients that depend on table.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/require-concurrent-index-creation">
                          require-concurrent-index-creation
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking writes to table while index is created.
                      </td>
                    </tr>
                    <tr>
                      <td style={{ wordBreak: "keep-all" }}>
                        <a href="/docs/require-concurrent-index-deletion">
                          require-concurrent-index-deletion
                        </a>
                      </td>
                      <td style={{ whiteSpace: "nowrap" }}>locking</td>
                      <td>
                        Prevent blocking reads/writes to table while index is
                        dropped.
                      </td>
                    </tr>
                  </table>
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
