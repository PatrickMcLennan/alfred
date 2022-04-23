import * as yup from 'yup';

export type LoginDto = {
  email: string;
  password: string;
};

export type LoginFormType = {
  email: string;
  password: string;
};

export const errorMessages = {
  noEmail: `An email is required`,
  invalidEmail: `This is an invalid email`,
  noPassword: `A password is required`,
  invalidPassword: `Must Contain 12 Characters, One Uppercase, One Lowercase, One Number and One Special Case Character`,
};

export const loginValidator: yup.SchemaOf<LoginDto> = yup.object().shape({
  email: yup.string().email(errorMessages.invalidEmail).required(errorMessages.noEmail),
  password: yup
    .string()
    .min(12, errorMessages.invalidPassword)
    .required(errorMessages.noPassword)
    .matches(
      // eslint-disable-next-line
      /^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#\$%\^&\*])(?=.{12,})/,
      errorMessages.invalidPassword
    ),
});
