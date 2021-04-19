module.exports = {
  title: "Squawk — a linter for Postgres migrations",
  tagline: "Reveal blocking schema changes with the Squawk CLI and GitHub App.",
  url: "https://squawkhq.com",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/owl.png",
  organizationName: "sbdchd", // Usually your GitHub org/user name.
  projectName: "squawk", // Usually your repo name.
  themeConfig: {
    prism: {
      theme: require("prism-react-renderer/themes/oceanicNext"),
    },
    algolia: {
      apiKey: process.env.ALGOLIA_API_KEY,
      indexName: process.env.ALGOLIA_INDEX_NAME,
    },
    sidebarCollapsible: false,
    colorMode: {
      defaultMode: "light",
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    image: "img/squawk-wordmark.png",
    metadatas: [
      {
        name: "title",
        content: "Squawk — a linter for Postgres migrations",
      },
      {
        name: "description",
        content:
          "Reveal blocking schema changes with the Squawk CLI and GitHub App.",
      },
    ],
    navbar: {
      title: "Squawk",
      logo: {
        alt: "Squawk Logo",
        src: "img/owl.png",
      },
      items: [
        {
          to: "docs/",
          label: "Docs",
          position: "left",
        },
        {
          to: "docs/rules",
          activeBasePath: "docs/rules",
          label: "Rules",
          position: "left",
        },
        {
          to: "docs/",
          label: "Quick Start",
          position: "right",
        },
        {
          href: "https://github.com/sbdchd/squawk",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            {
              label: "Quick Start",
              to: "docs/",
            },
            {
              label: "Rules",
              to: "docs/rules",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/sbdchd/squawk",
            },
            {
              label: "Changelog",
              href: "https://github.com/sbdchd/squawk/blob/master/CHANGELOG.md",
            },
            {
              label: "Help",
              href: "https://github.com/sbdchd/squawk/issues/new",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Squawk Authors.`,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          editUrl: "https://github.com/sbdchd/squawk/edit/master/docs/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
}
