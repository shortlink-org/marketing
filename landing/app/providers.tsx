'use client'

import React, { useEffect, useState } from 'react'
import { DEFAULT_ONLOAD_NAME, DEFAULT_SCRIPT_ID, SCRIPT_URL } from '@marsidev/react-turnstile'
import { Turnstile } from '@marsidev/react-turnstile'
import CssBaseline from '@mui/material/CssBaseline'
import { AppRouterCacheProvider } from '@mui/material-nextjs/v14-appRouter'
import { ThemeProvider as MuiThemeProvider, createTheme } from '@mui/material/styles'
import InitColorSchemeScript from '@mui/material/InitColorSchemeScript'
import { theme as baseTheme } from '@shortlink-org/ui-kit/dist/theme/theme'
import Script from 'next/script'
import { ThemeProvider as NextThemeProvider } from 'next-themes'

// TODO: faro has old peer dependencies, so we need to fix it before enabling it
//
// import { TracingInstrumentation } from '@grafana/faro-web-tracing'
// import { getWebInstrumentations, initializeFaro } from '@grafana/faro-web-sdk'
//
// initializeFaro({
//   url: process.env.NEXT_PUBLIC_FARO_URI,
//   app: {
//     name: process.env.NEXT_PUBLIC_SERVICE_NAME,
//     version: process.env.NEXT_PUBLIC_GIT_TAG,
//     environment: 'production',
//   },
//   instrumentations: [
//     // Mandatory, overwriting the instrumentations array would cause the default instrumentations to be omitted
//     ...getWebInstrumentations(),
//
//     // Initialization of the tracing package.
//     // This package is optional because it increases the bundle size noticeably. Only add it if you want tracing data.
//     new TracingInstrumentation(),
//   ],
// })

// Create theme with manual color scheme selector
const theme = createTheme({
  ...baseTheme,
  cssVariables: {
    colorSchemeSelector: 'class',
  },
})

// @ts-ignore
export function Providers({ children, ...props }) {
  const siteKey = process.env.NEXT_PUBLIC_CLOUDFLARE_SITE_KEY
  const isCaptchaEnabled = Boolean(siteKey)

  const [isCaptcha, setIsCaptcha] = useState(!isCaptchaEnabled)
  const [turnstileVersion, setTurnstileVersion] = useState(0)

  useEffect(() => {
    if (!isCaptchaEnabled) {
      return
    }

    const handlePageShow = (event: PageTransitionEvent) => {
      if (!event.persisted) {
        return
      }

      // BFCache restore can return with an expired/used token.
      // Remount widget to force a fresh token after back/forward navigation.
      setIsCaptcha(false)
      setTurnstileVersion((version) => version + 1)
    }

    window.addEventListener('pageshow', handlePageShow)

    return () => {
      window.removeEventListener('pageshow', handlePageShow)
    }
  }, [isCaptchaEnabled])

  return (
    <AppRouterCacheProvider>
      <NextThemeProvider enableSystem attribute="class" defaultTheme={'light'}>
        <MuiThemeProvider theme={theme}>
          {isCaptchaEnabled && (
            <Script id={DEFAULT_SCRIPT_ID} src={`${SCRIPT_URL}?onload=${DEFAULT_ONLOAD_NAME}`} strategy="afterInteractive" />
          )}
          <InitColorSchemeScript />

          <div className="flex m-auto text-black dark:bg-gray-800 dark:text-white flex-col">
            {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
            <CssBaseline />

            {!isCaptcha && isCaptchaEnabled && (
              <div style={{ position: 'absolute', top: '1em', left: '1em' }}>
                <Turnstile
                  key={turnstileVersion}
                  siteKey={siteKey as string}
                  injectScript={false}
                  className="captcha"
                  options={{
                    refreshExpired: 'auto',
                    refreshTimeout: 'auto',
                  }}
                  onSuccess={() => setIsCaptcha(true)}
                  onExpire={() => setIsCaptcha(false)}
                  onError={() => setIsCaptcha(false)}
                />
              </div>
            )}

            <div style={{ marginTop: isCaptcha ? 0 : '6em' }}>
              {isCaptcha && children}
            </div>
          </div>
        </MuiThemeProvider>
      </NextThemeProvider>
    </AppRouterCacheProvider>
  )
}
