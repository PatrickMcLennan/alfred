import { FC } from 'react';
import { Box } from '@mui/system';
import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import { Header, Footer, MuiTheme, Router } from './components';
import { Navbar } from './components/Navbar';

export const App: FC = () => {
  return (
    <MuiTheme>
      <BrowserRouter>
        <Header />
        <Box className="main-wrapper">
          <Navbar />
          <Box className="main" component="main">
            <Router />
          </Box>
        </Box>
        <Footer />
      </BrowserRouter>
    </MuiTheme>
  );
};
