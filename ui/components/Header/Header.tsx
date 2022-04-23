import React, { FC } from 'react';
import { Box, Container, Typography } from '@mui/material';
import { Link } from 'react-router-dom';

type Props = {
  isLoggedIn: boolean;
};

export const Header: FC<Props> = ({ isLoggedIn }) => {
  return (
    <Container className="header" component="header" maxWidth="lg">
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
