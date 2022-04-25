import React, { FC, ReactNode } from 'react';
import { Button as MuiButton, ButtonProps } from '@mui/material';

interface Props extends ButtonProps {
  loading?: boolean;
  children: ReactNode;
}

export const Button: FC<Props> = ({ children, disabled, loading, ...props }) => {
  return (
    <MuiButton disabled={disabled || loading} {...props}>
      {children}
    </MuiButton>
  );
};
