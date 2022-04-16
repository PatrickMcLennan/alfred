import React, { FC } from 'react';
import { Container, Typography } from '@mui/material';

const currentYear = new Date().getFullYear();

export const Footer: FC = () => {
  return (
    <Container component="footer" data-testid="footer" maxWidth="lg">
      <Typography>Copyright {currentYear} Patrick McLennan</Typography>
    </Container>
  );
};
