import React, { FC } from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import { Crypto, Home, Login, Wallpapers } from '../../pages';
import { useUser } from '../../stores';

export const Router: FC = () => {
  const { isLoggedIn } = useUser();

  return (
    <Routes>
      <Route path="/" element={isLoggedIn ? <Home /> : <Navigate replace to="/login" />} />
      <Route path="/login" element={<Login />} />
      <Route path="/crypto" element={isLoggedIn ? <Crypto /> : <Navigate replace to="/login" />} />
      <Route path="/wallpapers" element={isLoggedIn ? <Wallpapers /> : <Navigate replace to="/login" />} />
    </Routes>
  );
};
