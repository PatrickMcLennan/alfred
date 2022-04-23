import '@testing-library/jest-dom';
import React from 'react';
import { screen, render } from '@testing-library/react';
import { Login } from '../Login';
import { MuiTheme } from '../../components/MuiTheme';

describe(`<Login />`, () => {
  beforeEach(() =>
    render(
      <MuiTheme>
        <Login />
      </MuiTheme>
    )
  );

  it(`should render an H1 with the site name`, () =>
    expect(screen.getByRole(`heading`, { level: 1 }).textContent).toBe(`alfred`));

  it(`should render the <LoginForm />`, () => screen.getByTestId(`login-form`));
});
