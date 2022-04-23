# alfred

A cloud based orchestrator + store for all of my home automation and media needs.  Alfred runs on AWS using the TypeScript CDK for IaC and Rust SDKs for lambdas.  GUI is React.

## Contributing
<hr />

### UI

The UI is built with TypeScript + React and has a custom webpack configuration to handle different builds.  [webpack-dev-server](https://webpack.js.org/configuration/dev-server) is configured for local development with linting + hot reloading, and a prod configuration will compile, minify, cache and bundle all new code for production.  See the `*:ui:*` commands within the `package.json` file.

#### What you need

 - [Node.js](https://nodejs.org/en/download/)
 - [NVM](https://github.com/nvm-sh/nvm)

##### Getting Started

 - Run `nvm use` in the root directory, and make sure `node -v` outputs `v17.8.0` after that. 
 - Once you're using `v17.8.0`, run `npm install` to install the projects dependencies.  


All UI calls are proxied through [webpack-dev-server](https://webpack.js.org/configuration/dev-server/#devserverproxy) to our AWS domain.  You'll need to generate appropriate keys to run `https` locally and use our cookie auth system. Follow [these steps](https://gist.github.com/pgilad/63ddb94e0691eebd502deee207ff62bd) and place the generated files in the root directory.