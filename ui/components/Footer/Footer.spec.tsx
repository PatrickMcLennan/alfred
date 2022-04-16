import React from 'react';
import { screen, render } from '@testing-library/react';
import { Footer } from '.';
import { MuiTheme } from '../MuiTheme';

describe(`<Footer />`, () => {
  beforeEach(() =>
    render(
      <MuiTheme>
        <Footer />
      </MuiTheme>
    )
  );

  it(`should display a copyright message with the current year`, () => {
    const currentYear = new Date().getFullYear();
    screen.getByText(`Copyright ${currentYear} Patrick McLennan`);
  });
});
