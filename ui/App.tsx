import React, { FC } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { Header, Footer, MuiTheme } from './components';
import { Crypto, Home, Wallpapers } from './pages';

export const App: FC = () => {
  return (
    <MuiTheme>
      <Router>
        <Header />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/wallpapers" element={<Wallpapers />} />
          <Route path="/crypto" element={<Crypto />} />
        </Routes>
        <Footer />
      </Router>
    </MuiTheme>
  );
};
