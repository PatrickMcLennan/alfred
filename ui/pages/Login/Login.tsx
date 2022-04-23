import { useCallback, useState } from 'react';
import { Container } from '@mui/material';
import React, { FC } from 'react';
import { LoginDto } from '../../validators/login.validator';
import { LoginForm } from '../../components';
import { axiosClient } from '../../clients';
import { useNavigate } from 'react-router-dom';
import { useUser } from '../../stores';

export const Login: FC = () => {
  const [loading, setLoading] = useState(false);
  const { userAuthEvent } = useUser();
  const navigate = useNavigate();

  const onSubmit = useCallback(
    async (user: LoginDto) => {
      setLoading(true);
      try {
        await axiosClient({
          method: `POST`,
          url: `/auth/login`,
          data: user,
        }).then(() => {
          userAuthEvent(true);
          navigate(`/`);
        });
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    },
    [navigate, userAuthEvent]
  );

  return (
    <Container maxWidth="lg">
      <LoginForm loading={loading} onSubmit={onSubmit} />
    </Container>
  );
};
