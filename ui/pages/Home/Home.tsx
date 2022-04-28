import React, { FC, useCallback } from 'react';
import { Box, Theme } from '@mui/material';
import { WallpaperModal, ImageSwiper } from '../../components';
import { Outlet, Route, Routes, useNavigate } from 'react-router-dom';
import { ImageType } from '../../../lib/ts';
import { Helmet } from 'react-helmet';

const sx = {
  container: {
    padding: (theme: Theme) => theme.spacing(2),
  },
  swiperWrapper: {
    '&:not(:first-of-type)': {
      marginTop: (theme: Theme) => theme.spacing(2),
    },
  },
} as const;

export const Home: FC = () => {
  const navigate = useNavigate();

  const toggleWallpaper = useCallback(
    (newSk?: string) => (newSk ? navigate(`/home/wallpaper/${newSk}`) : navigate(`/home`)),
    [navigate]
  );

  return (
    <>
      <Helmet>
        <title>alfred</title>
        <meta name="description" content="Home page" />
      </Helmet>
      <Box sx={sx.container}>
        <Box sx={sx.swiperWrapper}>
          <ImageSwiper focus={toggleWallpaper} title="Widescreen Wallpapers" variant={ImageType.WIDESCREEN_WALLPAPER} />
        </Box>
        <Box sx={sx.swiperWrapper}>
          <ImageSwiper focus={toggleWallpaper} title="Amoled Backgrounds" variant={ImageType.AMOLED_BACKGROUND} />
        </Box>
        <Routes>
          <Route path="/wallpaper/:sk" element={<WallpaperModal onClose={() => toggleWallpaper()} />} />
        </Routes>
        <Outlet />
      </Box>
    </>
  );
};
