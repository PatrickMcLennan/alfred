import React, { FC, ReactNode } from 'react';
import { CssBaseline, createTheme, ThemeProvider } from '@mui/material';

type Props = {
  children: ReactNode;
};

const theme = createTheme({});

export const MuiTheme: FC<Props> = ({ children }) => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  );
};
