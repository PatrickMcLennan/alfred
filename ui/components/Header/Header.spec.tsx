import React from 'react';
import { screen, render } from '@testing-library/react';
import { Header } from '.';
import { MuiTheme } from '../MuiTheme';
import { BrowserRouter } from 'react-router-dom';

describe(`<Header />`, () => {
  beforeEach(() =>
    render(
      <BrowserRouter>
        <MuiTheme>
          <Header />
        </MuiTheme>
      </BrowserRouter>
    )
  );

  it('should display a menu with a link to the wallpapers page', () => {
    const wallpapersLink = screen.getByTestId('wallpapers-link');
    expect(wallpapersLink.getAttribute('href')).toBe('/wallpapers');
  });

  it('should display a menu with a link to the crypto page', () => {
    const cryptoLink = screen.getByTestId('crypto-link');
    expect(cryptoLink.getAttribute('href')).toBe('/crypto');
  });
});
