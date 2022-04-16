import React, { FC } from 'react';
import { Box, Container, Typography } from '@mui/material';
import { Link } from 'react-router-dom';

export const Header: FC = () => {
  return (
    <Container component="header" maxWidth="lg">
      <Box>
        <Typography component={Link} data-testid="wallpapers-link" to="/wallpapers">
          Wallpapers
        </Typography>
        <Typography component={Link} data-testid="crypto-link" to="/crypto">
          Crypto
        </Typography>
      </Box>
    </Container>
  );
};
