import { Divider, Stack, Typography, useTheme } from '@mui/material'
import Button from '@mui/material/Button'
import Link from 'next/link'
import React from 'react'

interface Card {
  name: string
  url: string
}

interface TabContentProps {
  title: string
  cards: Card[]
}

const TabContent: React.FC<TabContentProps> = ({ title, cards }) => {
  const theme = useTheme()

  return (
    <div className="my-2 mx-5 max-w-4xl mx-auto">
      <Typography 
        variant="h4" 
        component="h2" 
        align="center"
        sx={{ 
          fontWeight: 'bold',
          fontSize: { xs: '1.5rem', sm: '2rem' },
          margin: '1em 0',
          color: '#6b7280'
        }}
      >
        {title}
      </Typography>

      <Stack
        spacing={{ xs: 1, sm: 1, md: 2 }}
        direction={{ xs: 'column', sm: 'row' }}
        divider={<Divider orientation="vertical" flexItem />}
        mt={2}
        justifyContent="center"
        alignItems="center"
        useFlexGap
        flexWrap="wrap"
        sx={{ padding: '1em 1em' }}
      >
        {cards.map((card) => (
          <Link href={card.url} key={card.url} passHref>
            <Button
              variant="outlined"
              size="large"
              sx={{
                minWidth: 160,
                color: theme.palette.text.primary,
                borderColor: theme.palette.divider,
                '&:hover': {
                  borderColor: theme.palette.primary.main,
                  backgroundColor: theme.palette.action.hover,
                },
              }}
            >
              {card.name}
            </Button>
          </Link>
        ))}
      </Stack>
    </div>
  )
}

export default TabContent
