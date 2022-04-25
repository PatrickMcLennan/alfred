import React, { FC, useCallback } from 'react';
import { Container } from '@mui/material';
import { WallpaperModal, WallpaperSwiper } from '../../components';
import { Outlet, Route, Routes, useNavigate, useParams } from 'react-router-dom';

type Params = {
  sk?: string;
};

export const Home: FC = () => {
  const navigate = useNavigate();
  const { sk } = useParams<Params>();

  const toggleWallpaper = useCallback(
    (newSk?: string) => (newSk ? navigate(`/home/wallpaper/${newSk}`) : navigate(`/home`)),
    [navigate]
  );

  return (
    <Container maxWidth="lg">
      <WallpaperSwiper focus={toggleWallpaper} />
      <Routes>
        <Route path="/wallpaper/:sk" element={<WallpaperModal onClose={() => toggleWallpaper()} />} />
      </Routes>
      <Outlet />
    </Container>
  );
};
