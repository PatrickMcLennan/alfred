import { FC, StrictMode, useRef } from 'react';
import { Box } from '@mui/system';
import { AxiosRequestConfig } from 'axios';
import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import { SWRConfig } from 'swr';
import { axiosFetcher } from './clients';
import { Header, Footer, MuiTheme } from './components';
import { Router } from './components';
import { useUser } from './stores';

type Props = {
  isLoggedIn: boolean;
};

export const App: FC<Props> = ({ isLoggedIn }) => {
  const firstRender = useRef<boolean>(true);
  const { userAuthEvent } = useUser();

  if (firstRender.current) {
    userAuthEvent(isLoggedIn);
    firstRender.current = false;
  }

  return (
    <StrictMode>
      <SWRConfig
        value={{
          fetcher: (config: AxiosRequestConfig) => axiosFetcher(config).then(({ data }) => data),
        }}
      >
        <MuiTheme>
          <BrowserRouter>
            <Header />
            <Box className="main" component="main">
              <Router />
            </Box>
            <Footer />
          </BrowserRouter>
        </MuiTheme>
      </SWRConfig>
    </StrictMode>
  );
};
