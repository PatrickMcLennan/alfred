import { Box, IconButton, Theme } from '@mui/material';
import React, { FC } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import WallpaperIcon from '@mui/icons-material/Wallpaper';
import ShowChartIcon from '@mui/icons-material/ShowChart';
import HomeIcon from '@mui/icons-material/Home';

const sx = {
  active: {
    backgroundColor: 'white',
    transform: 'scale(1.1)',

    '& svg': {
      color: (theme: Theme) => theme.palette.background.default,
    },

    '&:hover svg': {
      color: 'white',
    },
  },
  buttonBox: {
    display: 'flex',
    justifyContent: 'flex-start',
    alignItems: 'center',
    '&:not(:last-of-type)': {
      marginBottom: (theme: Theme) => theme.spacing(2),
    },
  },
  expanded: {
    backgroundColor: `rgba(255, 255, 255, .25)`,
    padding: (theme: Theme) => theme.spacing(4, 2),

    '& .button-label': {
      opacity: 1,
      visibility: `visible`,
      width: 'auto',
    },
  },
  nav: {
    display: 'flex',
    justifyContent: 'flex-start',
    alignItems: 'flex-start',
    flexDirection: 'column',
    gridArea: 'navbar',
    borderRight: '1px solid white',
    padding: (theme: Theme) => theme.spacing(4, 1),
    transition: (theme: Theme) =>
      `padding .175s ${theme.transitions.easing.easeInOut}, transform .175s ${theme.transitions.easing.easeInOut}, background-color .175s ${theme.transitions.easing.easeInOut}`,

    '&:hover': {
      backgroundColor: `rgba(255, 255, 255, .15)`,
      cursor: 'pointer',
      padding: (theme: Theme) => theme.spacing(4, 2),
    },
  },
} as const;

export const Navbar: FC = () => {
  const isLoggedIn = !!document.cookie.match(/alfred_is_logged_in=true/)?.length;
  const navigate = useNavigate();
  const { pathname } = useLocation();

  if (!isLoggedIn) {
    return <></>;
  }

  return (
    <Box component="nav" sx={sx.nav}>
      <Box sx={sx.buttonBox}>
        <IconButton
          aria-label="Go to Home page"
          data-testid="home-link"
          onClick={(e) => {
            e.stopPropagation();
            navigate('/home');
          }}
          sx={{ ...sx.buttonBox, ...(pathname === '/home' ? sx.active : {}) }}
        >
          <HomeIcon />
        </IconButton>
      </Box>
      <Box sx={sx.buttonBox}>
        <IconButton
          aria-label="Go to Wallpapers page"
          data-testid="wallpapers-link"
          onClick={(e) => {
            e.stopPropagation();
            navigate('/wallpapers');
          }}
          sx={{ ...sx.buttonBox, ...(pathname === '/wallpapers' ? sx.active : {}) }}
        >
          <WallpaperIcon />
        </IconButton>
      </Box>
      <Box sx={sx.buttonBox}>
        <IconButton
          aria-label="Go to Investments page"
          data-testid="investments-link"
          onClick={(e) => {
            e.stopPropagation();
            navigate('/investments');
          }}
          sx={{ ...sx.buttonBox, ...(pathname === '/investments' ? sx.active : {}) }}
        >
          <ShowChartIcon />
        </IconButton>
      </Box>
    </Box>
  );
};
