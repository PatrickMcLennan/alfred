import React, { FC, useEffect } from 'react';
import { Container } from '@mui/material';
import { axiosClient } from '../../clients';
import { Helmet } from 'react-helmet';

export const Wallpapers: FC = () => {
  useEffect(() => {
    axiosClient({
      method: 'POST',
      url: `/wallpapers`,
    })
      .then(console.log)
      .catch(console.error);
  }, []);
  return (
    <>
      <Helmet>
        <title>alfred | Wallpapers</title>
        <meta name="description" content="View all Wallpapers" />
      </Helmet>
      <Container maxWidth="lg">This is the wallpapers page This is the wallpapers page</Container>
    </>
  );
};
