import React, { FC } from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import { Investments, Home, Login, Wallpapers } from '../../pages';

export const Router: FC = () => {
  const isLoggedIn = !!document.cookie.match(/alfred_is_logged_in=true/)?.length;

  return (
    <Routes>
      <Route path="/home/*" element={isLoggedIn ? <Home /> : <Navigate replace to="/login" />} />
      <Route path="/login" element={isLoggedIn ? <Navigate replace to="/home" /> : <Login />} />
      <Route path="/investments" element={isLoggedIn ? <Investments /> : <Navigate replace to="/login" />} />
      <Route path="/wallpapers" element={isLoggedIn ? <Wallpapers /> : <Navigate replace to="/login" />} />
      <Route path="/" element={isLoggedIn ? <Navigate replace to="/home" /> : <Navigate replace to="/login" />} />
    </Routes>
  );
};
