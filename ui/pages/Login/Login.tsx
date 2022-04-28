import { useCallback, useState } from 'react';
import { Container, Theme, Typography } from '@mui/material';
import React, { FC } from 'react';
import { LoginDto } from '../../validators/login.validator';
import { LoginForm } from '../../components';
import { axiosClient } from '../../clients';
import { useNavigate } from 'react-router-dom';
import { Helmet } from 'react-helmet';

const sx = {
  container: {
    margin: '15vh auto 0 auto',
    flex: 1,
  },
  h1: {
    marginBottom: (theme: Theme) => theme.spacing(2),
  },
} as const;

export const Login: FC = () => {
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const onSubmit = useCallback(
    async (user: LoginDto) => {
      setLoading(true);
      try {
        await axiosClient({
          method: `POST`,
          url: `/auth/login`,
          data: user,
        }).then(() => navigate(`/home`));
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    },
    [navigate]
  );

  return (
    <>
      <Helmet>
        <title>alfred | Login</title>
        <meta name="description" content="Log in" />
      </Helmet>
      <Container maxWidth="sm" sx={sx.container}>
        <Typography component="h1" sx={sx.h1} variant="h1">
          alfred
        </Typography>
        <LoginForm loading={loading} onSubmit={onSubmit} />
      </Container>
    </>
  );
};
