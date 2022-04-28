import React, { FC } from 'react';
import { Container } from '@mui/material';
import { Helmet } from 'react-helmet';

export const Investments: FC = () => {
  return (
    <>
      <Helmet>
        <title>alfred | Investments</title>
        <meta name="description" content="Investments info" />
      </Helmet>
      <Container maxWidth="lg">This is the investments page This is the investments page</Container>
    </>
  );
};
