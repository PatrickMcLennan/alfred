/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  transform: {
    '.+\\.(css|styl|less|sass|scss)$': 'jest-css-modules-transform',
    '\\.(png|jpg|jpeg)$': 'jest-file-loader',
  },
  transformIgnorePatterns: ['/node_modules/(?!antd|@ant-design|rc-.+?|@babel/runtime).+(js|jsx)$'],
  globals: {
    'ts-jest': {
      tsConfig: '<rootDir>/tsconfig.json',
    },
  },
  moduleNameMapper: {
    clients: '<rootDir>/ui/src/clients',
    components: '<rootDir>/ui/src/components',
    pages: '<rootDir>/ui/src/pages',
    validators: '<rootDir>/ui/src/validators',
  },
};
