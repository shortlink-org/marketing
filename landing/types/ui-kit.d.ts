declare module '@shortlink-org/ui-kit' {
  import type { ComponentType, ReactNode } from 'react'

  export const theme: any

  export const ToggleDarkMode: ComponentType<{ id?: string }>

  export const ScrollToTopButton: ComponentType<{
    scrollContainer?: Window | HTMLElement | null
    variant?: 'solid' | 'glass'
    label?: string
    scrollThreshold?: number
    iconSize?: number
    enableGlobalHotkey?: boolean
  }>

  export const StatCard: ComponentType<{
    label: ReactNode
    value: ReactNode
    change?: ReactNode
    tone?: 'neutral' | 'accent' | 'success' | 'warning' | 'danger'
    className?: string
    labelClassName?: string
    valueClassName?: string
    changeClassName?: string
  }>

  export const Footer: ComponentType<{
    className?: string
    contained?: boolean
    rounded?: boolean
    withTopMargin?: boolean
    contentClassName?: string
    links?: Array<{ id?: string; label: string; href: string; target?: string }>
    socialLinks?: Array<{
      name: string
      href: string
      iconPath: string
      viewBox?: string
    }>
    copyright?: ReactNode
    logoSlot?: ReactNode
    description?: ReactNode
    LinkComponent?: ComponentType<any>
  }>
}
