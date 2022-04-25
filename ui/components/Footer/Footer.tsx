import React, { FC } from 'react';
import { Container, Theme, Typography } from '@mui/material';

const sx = {
  container: {
    textAlign: 'center',
    padding: (theme: Theme) => theme.spacing(4),
  },
} as const;

const currentYear = new Date().getFullYear();

export const Footer: FC = () => {
  return (
    <Container className="footer" component="footer" data-testid="footer" maxWidth="md" sx={sx.container}>
      <Typography>Copyright {currentYear} Patrick McLennan</Typography>
    </Container>
  );
};
