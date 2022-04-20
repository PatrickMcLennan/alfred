# authorizer

An authorizer lambda that intercepts incoming HTTP requests on specified API Gateway endpoints and checks for specific `httpOnly` authentication cookies from this app.  If found, unwraps the cookies, checks / refreshses validity of the tokens and accepts or denies the request.

To use, download the JSON Webkey for your user pool ([Read more here](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-2)) and store it as a `keys.json` file in this directory for reference.  Add the correct `e` + `n` values from the private key to the root level `.env` file (see `.env.example`).