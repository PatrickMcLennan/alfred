import React, { FC, useState } from 'react';
import { Box, IconButton, Theme } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { axiosClient } from '../../clients';
import LogoutIcon from '@mui/icons-material/Logout';

import NotificationsIcon from '@mui/icons-material/Notifications';
import PersonIcon from '@mui/icons-material/Person';

const sx = {
  header: {
    display: 'flex',
    justifyContent: 'flex-end',
    alignItem: 'center',
    flexDirection: 'row',
    padding: (theme: Theme) => theme.spacing(1, 4),
    borderBottom: '1px solid white',
    gridArea: 'header',
    gridColumn: '1 / -1',
  },
  hide: {
    display: 'none',
  },
} as const;

export const Header: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const navigate = useNavigate();
  const isLoggedIn = !!document.cookie.match(/alfred_is_logged_in=true/)?.length;

  const logout = async () => {
    setLoading(true);
    try {
      await axiosClient({
        method: 'POST',
        url: '/auth/logout',
      }).then(() => navigate(`/login`));
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box className="header" component="header" sx={isLoggedIn ? sx.header : sx.hide}>
      {isLoggedIn && (
        <IconButton data-testid="logout-button" disabled={loading} onClick={logout}>
          <LogoutIcon />
        </IconButton>
      )}
    </Box>
  );
};
