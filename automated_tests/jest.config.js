module.exports = {
    roots: [
      '<rootDir>',
    ],
    preset: 'ts-jest',
    testEnvironment: 'node',
    testMatch: ['**/?(*.)+(spec|test).ts'],
    testTimeout: 30000,
  };
  