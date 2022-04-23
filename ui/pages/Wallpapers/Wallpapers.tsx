import React, { FC, useEffect } from 'react';
import { Container } from '@mui/material';
import { axiosClient } from '../../clients';

export const Wallpapers: FC = () => {
  useEffect(() => {
    axiosClient({
      method: 'POST',
      url: `/wallpapers`,
    })
      .then(console.log)
      .catch(console.error);
  }, []);
  return <Container maxWidth="lg">This is the wallpapers page This is the wallpapers page</Container>;
};
