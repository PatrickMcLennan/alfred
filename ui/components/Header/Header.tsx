import React, { FC, useState } from 'react';
import { Avatar, Box, IconButton, ListItemIcon, Menu, MenuItem, Theme } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { axiosClient } from '../../clients';
import LogoutIcon from '@mui/icons-material/Logout';

import NotificationsIcon from '@mui/icons-material/Notifications';
import PersonIcon from '@mui/icons-material/Person';
import SettingsIcon from '@mui/icons-material/Settings';

const sx = {
  avatarActive: {
    backgroundColor: 'white',
    color: 'black',
  },
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
  paper: {
    overflow: 'visible',
    filter: 'drop-shadow(0px 2px 8px rgba(0,0,0,0.32))',
    border: `1px solid white`,
    mt: 1.5,
    '& .MuiAvatar-root': {
      width: 32,
      height: 32,
      ml: -0.5,
      mr: (theme: Theme) => theme.spacing(4),
    },
    '&:before': {
      content: '""',
      display: 'block',
      position: 'absolute',
      top: 0,
      right: 14,
      width: 10,
      height: 10,
      borderLeft: `1px solid white`,
      borderTop: `1px solid white`,
      bgcolor: 'background.paper',
      transform: 'translateY(-50%) rotate(45deg)',
      zIndex: 0,
    },
  },
} as const;

export const Header: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const openMenu = ({ currentTarget }: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(currentTarget);
  };
  const closeMenu = () => {
    setAnchorEl(null);
  };
  const navigate = useNavigate();
  const isLoggedIn = !!document.cookie.match(/alfred_is_logged_in=true/)?.length;

  const toProfilePage = () => navigate(`/profile`);
  const toSettingsPage = () => navigate(`/settings`);

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
        <>
          <IconButton
            aria-controls={open ? 'account-menu' : undefined}
            aria-haspopup="true"
            aria-expanded={open ? 'true' : undefined}
            data-testid="logout-button"
            disabled={loading}
            onClick={openMenu}
          >
            <Avatar sx={{ ...(open ? sx.avatarActive : {}) }}>P</Avatar>
          </IconButton>
          <Menu
            anchorEl={anchorEl}
            id="account-menu"
            open={open}
            onClose={closeMenu}
            onClick={closeMenu}
            PaperProps={{
              elevation: 0,
              sx: sx.paper,
            }}
            transformOrigin={{ horizontal: 'right', vertical: 'top' }}
            anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
          >
            <MenuItem onClick={logout}>
              <ListItemIcon>
                <LogoutIcon fontSize="small" />
              </ListItemIcon>
              Logout
            </MenuItem>
            <MenuItem onClick={toProfilePage}>
              <ListItemIcon>
                <PersonIcon fontSize="small" />
              </ListItemIcon>
              Profile
            </MenuItem>
            <MenuItem onClick={toSettingsPage}>
              <ListItemIcon>
                <SettingsIcon fontSize="small" />
              </ListItemIcon>
              Settings
            </MenuItem>
          </Menu>
        </>
      )}
    </Box>
  );
};
