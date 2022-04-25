import React from 'react';
import { Box, FormHelperText, FormControl, InputLabel, Input, CircularProgress, Theme } from '@mui/material';
import { useForm, Controller } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';
import { LoginDto, loginValidator } from '../../validators/login.validator';
import { Button } from '../Button';

type Props = {
  onSubmit: (_user: LoginDto) => unknown;
  disabled?: boolean;
  loading: boolean;
};

const sx = {
  form: {
    display: `flex`,
    justifyContent: 'flex-start',
    alignItems: 'stretch',
    flexDirection: 'column',
    gap: (theme: Theme) => theme.spacing(2),
  },
  formHelperText: {
    '&, & span': {
      height: `2rem`,
    },
  },
  submit: {
    marginTop: (theme: Theme) => theme.spacing(4),
    backgroundColor: (theme: Theme) => theme.palette.secondary.main,
    alignSelf: 'center',
  },
} as const;

export function LoginForm({ onSubmit, disabled, loading }: Props) {
  const {
    formState: { errors },
    handleSubmit,
    control,
  } = useForm<LoginDto>({
    defaultValues: {
      email: ``,
      password: ``,
    },
    resolver: yupResolver(loginValidator),
  });

  return (
    <Box
      className="login-form"
      data-testid="login-form"
      component="form"
      noValidate
      autoComplete="off"
      onSubmit={handleSubmit((user: LoginDto) => onSubmit(user))}
      sx={sx.form}
    >
      <Controller
        control={control}
        name="email"
        render={({ field }) => {
          const emailError = errors?.email;
          return (
            <FormControl error={!!emailError}>
              <InputLabel htmlFor="email">Email</InputLabel>
              <Input {...field} id="email" type="email" />
              <FormHelperText sx={sx.formHelperText} aria-hidden={!emailError}>
                {emailError ? emailError.message : ` `}
              </FormHelperText>
            </FormControl>
          );
        }}
      />
      <Controller
        control={control}
        name="password"
        render={({ field }) => {
          const passwordError = errors.password;
          return (
            <FormControl error={!!passwordError}>
              <InputLabel htmlFor="password">Password</InputLabel>
              <Input {...field} id="password" type="password" />
              <FormHelperText sx={sx.formHelperText} aria-hidden={!passwordError}>
                {passwordError ? passwordError.message : ` `}
              </FormHelperText>
            </FormControl>
          );
        }}
      />
      <Button disabled={loading || disabled} loading={loading} sx={sx.submit} type="submit">
        {loading ? <CircularProgress /> : 'Log In'}
      </Button>
    </Box>
  );
}
