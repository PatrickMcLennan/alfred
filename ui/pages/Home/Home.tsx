import React, { FC, useCallback } from 'react';
import { Box, Theme } from '@mui/material';
import { WallpaperModal, WallpaperSwiper } from '../../components';
import { Outlet, Route, Routes, useNavigate } from 'react-router-dom';

const sx = {
  container: {
    padding: (theme: Theme) => theme.spacing(2),
  },
} as const;

export const Home: FC = () => {
  const navigate = useNavigate();

  const toggleWallpaper = useCallback(
    (newSk?: string) => (newSk ? navigate(`/home/wallpaper/${newSk}`) : navigate(`/home`)),
    [navigate]
  );

  return (
    <Box sx={sx.container}>
      <WallpaperSwiper focus={toggleWallpaper} />
      <Routes>
        <Route path="/wallpaper/:sk" element={<WallpaperModal onClose={() => toggleWallpaper()} />} />
      </Routes>
      <Outlet />
    </Box>
  );
};
