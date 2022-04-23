import { StrictMode } from 'react';
import { Box } from '@mui/system';
import { AxiosRequestConfig } from 'axios';
import React from 'react';
import { BrowserRouter as Router, Navigate, Route, Routes } from 'react-router-dom';
import { SWRConfig } from 'swr';
import { axiosFetcher } from './clients';
import { Header, Footer, MuiTheme } from './components';
import { Crypto, Home, Login, Wallpapers } from './pages';

export const App = () => {
  const isLoggedIn = document.cookie.includes(`alfred_is_logged_in=true`);

  return (
    <StrictMode>
      <SWRConfig
        value={{
          fetcher: (config: AxiosRequestConfig) => axiosFetcher(config).then(({ data }) => data),
        }}
      >
        <MuiTheme>
          <Router>
            <Header isLoggedIn />
            <Box className="main" component="main">
              <Routes>
                <Route path="/" element={isLoggedIn ? <Home /> : <Navigate replace to="/login" />} />
                <Route path="/login" element={<Login />} />
                <Route path="/crypto" element={isLoggedIn ? <Crypto /> : <Navigate replace to="/login" />} />
                <Route path="/wallpapers" element={isLoggedIn ? <Wallpapers /> : <Navigate replace to="/login" />} />
              </Routes>
            </Box>
            <Footer />
          </Router>
        </MuiTheme>
      </SWRConfig>
    </StrictMode>
  );
};
