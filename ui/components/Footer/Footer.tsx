import React, { FC } from 'react';
import { Container, Theme, Typography } from '@mui/material';

const sx = {
  container: {
    textAlign: 'center',
    padding: (theme: Theme) => theme.spacing(2),
    gridArea: 'footer',
    gridColumnSpan: 2,
    borderTop: '1px solid white',
  },
  typography: {
    fontSize: `1.4rem`,
  },
} as const;

const currentYear = new Date().getFullYear();

export const Footer: FC = () => {
  return (
    <Container className="footer" component="footer" data-testid="footer" maxWidth="md" sx={sx.container}>
      <Typography sx={sx.typography}>Copyright {currentYear} Patrick McLennan</Typography>
    </Container>
  );
};
