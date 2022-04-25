import { FC, StrictMode, useRef } from 'react';
import { Box } from '@mui/system';
import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import { Header, Footer, MuiTheme, Router } from './components';
import { useUser } from './stores';

type Props = {
  isLoggedIn: boolean;
};

export const App: FC<Props> = ({ isLoggedIn }) => {
  const firstRender = useRef<boolean>(true);
  const { userAuthEvent } = useUser(({ userAuthEvent }) => ({ userAuthEvent }));

  if (firstRender.current) {
    userAuthEvent(isLoggedIn);
    firstRender.current = false;
  }

  return (
    <StrictMode>
      <MuiTheme>
        <BrowserRouter>
          <Header />
          <Box className="main" component="main">
            <Router />
          </Box>
          <Footer />
        </BrowserRouter>
      </MuiTheme>
    </StrictMode>
  );
};
