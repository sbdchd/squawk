module.exports = {
  title: "Squawk",
  tagline: "A linter for Postgres migrations",
  url: "https://your-docusaurus-test-site.com",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/owl.png",
  organizationName: "sbdchd", // Usually your GitHub org/user name.
  projectName: "squawk", // Usually your repo name.
  themeConfig: {
    sidebarCollapsible: false,
    colorMode: {
      defaultMode: "light",
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: "Squawk",
      logo: {
        alt: "Squawk Logo",
        src: "img/owl.png",
      },
      items: [
        {
          to: "docs/",
          activeBasePath: "docs",
          label: "Docs",
          position: "left",
        },
        {
          to: "docs/rules/",
          activeBasePath: "rules",
          label: "Rules",
          position: "left",
        },
        {
          href: "https://github.com/sbdchd/squawk",
          label: "Install",
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
              label: "Install",
              to: "docs/",
            },
            {
              label: "Quick Start",
              to: "docs/",
            },
            {
              label: "Rules",
              to: "docs/doc2/",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/facebook/docusaurus",
            },
            {
              label: "Changelog",
              href: "https://github.com/facebook/docusaurus",
            },
            {
              label: "Help",
              href: "https://github.com/facebook/docusaurus",
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
