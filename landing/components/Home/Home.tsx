'use client'

import Link from 'next/link'
import { useTheme, AppBar, Tabs, Tab } from '@mui/material'
import useMediaQuery from '@mui/material/useMediaQuery'
import React, { useState } from 'react'
import { Footer, ScrollToTopButton, StatCard, ToggleDarkMode } from '@shortlink-org/ui-kit'

import TabContent from '../TabContent'
import TabPanel from '../TabPanel'

function a11yProps(index: number) {
  return {
    id: `full-width-tab-${index}`,
    'aria-controls': `full-width-tabpanel-${index}`,
  }
}

const sections = [
  {
    label: 'ShortLink',
    groups: [
      {
        title: 'UI',
        cards: [
          { name: 'Next', url: '/next' },
          { name: 'ui-kit', url: 'https://ui-kit.shortlink.best' },
        ],
      },
      {
        title: 'Shortlink API',
        cards: [{ name: 'HTTP', url: '/api' }],
      },
    ],
  },
  {
    label: 'Shop',
    groups: [
      {
        title: 'Shop',
        cards: [
          { name: 'Shop', url: 'https://shop.shortlink.best' },
          { name: 'Admin', url: 'https://shop.shortlink.best/admin' },
          { name: 'Temporal', url: 'https://temporal.shortlink.best' },
          { name: 'Storybook', url: 'https://shop.shortlink.best/storybook' },
        ],
      },
    ],
  },
  {
    label: 'Infrastructure',
    groups: [
      {
        title: 'Infrastructure services',
        cards: [
          { name: 'RabbitMQ', url: '/rabbitmq/' },
          { name: 'Kafka', url: '/kafka-ui/' },
          { name: 'Keycloak', url: 'https://keycloak.shortlink.best' },
        ],
      },
      {
        title: 'Argo',
        cards: [
          { name: 'Argo CD', url: 'https://argo.shortlink.best' },
          { name: 'Argo Rollout', url: 'https://argo.shortlink.best/rollout' },
          { name: 'Argo Workflows', url: 'https://workflows.shortlink.best' },
        ],
      },
    ],
  },
  {
    label: 'Security',
    groups: [
      {
        title: 'Security',
        cards: [
          { name: 'Armosec', url: 'https://cloud.armosec.io/compliance/shortlink' },
          { name: 'KubeShark', url: 'https://kubeshark.shortlink.best' },
          { name: 'Kyverno', url: '/kyverno/#/' },
        ],
      },
    ],
  },
  {
    label: 'Observability',
    groups: [
      {
        title: 'Observability services',
        cards: [
          { name: 'Prometheus', url: '/prometheus' },
          { name: 'AlertManager', url: '/alertmanager' },
          { name: 'Grafana', url: 'https://grafana.shortlink.best' },
          { name: 'Pyroscope', url: 'https://pyroscope.shortlink.best' },
          { name: 'Testkube', url: 'https://testkube.shortlink.best' },
          { name: 'TraceTest', url: 'https://tracetest.shortlink.best' },
          { name: 'Status Page', url: 'https://status.shortlink.best' },
        ],
      },
    ],
  },
  {
    label: 'Docs',
    groups: [
      {
        title: 'Documentation and etc...',
        cards: [
          { name: 'GitHub', url: 'https://github.com/shortlink-org/shortlink' },
          { name: 'GitLab', url: 'https://gitlab.com/shortlink-org/shortlink/' },
          { name: 'Swagger API', url: 'https://shortlink-org.gitlab.io/shortlink/' },
          { name: 'Backstage', url: 'https://backstage.shortlink.best/' },
          { name: 'Landscape', url: 'https://landscape.shortlink.best/' },
        ],
      },
    ],
  },
] as const

const Home = () => {
  const theme = useTheme()
  const [value, setValue] = useState(0)

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue)
  }

  const appBarColor = theme.palette.mode === 'dark' ? 'inherit' : 'primary'
  const textColor = theme.palette.mode === 'dark' ? 'secondary' : 'inherit'
  // @ts-ignore
  const isMobile = useMediaQuery((props) => props.breakpoints.down('sm'))
  const totalGroups = sections.reduce((acc, section) => acc + section.groups.length, 0)
  const totalLinks = sections.reduce(
    (acc, section) => acc + section.groups.reduce((groupAcc, group) => groupAcc + group.cards.length, 0),
    0,
  )

  return (
    <>
      <div style={{ position: 'absolute', top: '1.5em', right: '6.5em' }}>
        <ToggleDarkMode id="ToggleDarkMode" />
      </div>

      <div className="mx-auto mt-12 max-w-5xl px-4">
        <div className="mb-6 grid gap-3 sm:grid-cols-3">
          <StatCard label="Sections" value={sections.length} change="catalog" tone="accent" />
          <StatCard label="Link groups" value={totalGroups} change="curated" tone="success" />
          <StatCard label="Quick links" value={totalLinks} change="live" tone="warning" />
        </div>

        <div className="relative flex flex-col overflow-hidden rounded-2xl bg-white bg-clip-border text-gray-700 shadow-lg">
        <AppBar position="static" id="menu" color={appBarColor} className="mt-[10em] md:mt-0">
          <Tabs
            value={value}
            onChange={handleChange}
            indicatorColor="secondary"
            textColor={textColor}
            variant={isMobile ? 'scrollable' : 'fullWidth'}
            aria-label="scrollable full width tabs example"
            selectionFollowsFocus
            scrollButtons="auto"
            allowScrollButtonsMobile
            className="md:max-w-3xl mx-auto"
          >
            {sections.map((section, index) => (
              <Tab key={section.label} label={section.label} {...a11yProps(index)} />
            ))}
          </Tabs>
        </AppBar>

        {sections.map((section, index) => (
          <TabPanel key={section.label} value={value} index={index} dir={theme.direction}>
            {section.groups.map((group) => (
              <TabContent key={group.title} title={group.title} cards={[...group.cards]} />
            ))}
          </TabPanel>
        ))}
        </div>

        <Footer
          contained={false}
          rounded={false}
          className="mt-6 rounded-2xl border border-black/5 shadow-sm dark:border-white/10"
          description="Routing hub for ShortLink services, developer tools, docs, and storefront touchpoints."
          links={[
            { label: 'UI Kit', href: 'https://ui-kit.shortlink.best', target: '_blank' },
            { label: 'Storybook', href: 'https://shop.shortlink.best/storybook', target: '_blank' },
            { label: 'GitHub', href: 'https://github.com/shortlink-org/shortlink', target: '_blank' },
            { label: 'Backstage', href: 'https://backstage.shortlink.best/', target: '_blank' },
          ]}
          socialLinks={[]}
          copyright={
            <span>
              Version: <strong>{process.env.NEXT_PUBLIC_GIT_TAG}</strong>
              {' | '}
              <Link
                href={(process.env.NEXT_PUBLIC_CI_PIPELINE_URL || '#') as any}
                style={{ color: 'inherit', textDecoration: 'underline' }}
              >
                Pipeline: <strong>{process.env.NEXT_PUBLIC_PIPELINE_ID}</strong>
              </Link>
            </span>
          }
        />
      </div>

      <ScrollToTopButton />
    </>
  )
}

export default Home
