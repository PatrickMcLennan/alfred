import React, { FC, useState } from 'react';
import { Box, Button, Container, Theme, Typography } from '@mui/material';
import { Link, useNavigate } from 'react-router-dom';
import { axiosClient } from '../../clients';
import { useUser } from '../../stores';

const sx = {
  container: {
    padding: (theme: Theme) => theme.spacing(4),
  },
} as const;

export const Header: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const navigate = useNavigate();
  const { isLoggedIn, userAuthEvent } = useUser(({ isLoggedIn, userAuthEvent }) => ({ isLoggedIn, userAuthEvent }));

  const logout = async () => {
    setLoading(true);
    try {
      await axiosClient({
        method: 'POST',
        url: '/auth/logout',
      }).then(() => {
        userAuthEvent(false);
        navigate(`/login`);
      });
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container className="header" component="header" maxWidth="md" sx={sx.container}>
      <Box>
        {isLoggedIn && (
          <>
            <Typography component={Link} data-testid="wallpapers-link" to="/wallpapers">
              Wallpapers
            </Typography>
            <Typography component={Link} data-testid="crypto-link" to="/crypto">
              Crypto
            </Typography>
            <Typography component={Button} data-testid="logout-button" disabled={loading} onClick={logout}>
              Logout
            </Typography>
          </>
        )}
      </Box>
    </Container>
  );
};
