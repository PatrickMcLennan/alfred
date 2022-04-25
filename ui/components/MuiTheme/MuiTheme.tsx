import React, { FC, ReactNode } from 'react';
import { CssBaseline, createTheme, ThemeProvider } from '@mui/material';

type Props = {
  children: ReactNode;
};
// https://mui.com/material-ui/customization/palette/
const { palette, spacing, typography } = createTheme({});

const theme = createTheme({
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          '&.MuiButton-root': {
            backgroundColor: palette.info.dark,
            color: 'white',
            fontSize: '1.8rem',
            textTransform: 'none',
            padding: spacing(2, 4),
            '&:hover, &:focus, &:active': {
              backgroundColor: palette.info.main,
            },
            '&:disabled': {
              cursor: 'not-allowed',
              pointerEvents: 'auto',
              backgroundColor: palette.divider,
            },
          },
        },
      },
    },
  },
  palette: {
    mode: `dark`,
  },
  typography: {
    htmlFontSize: 10,
  },
});

export const MuiTheme: FC<Props> = ({ children }) => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  );
};
