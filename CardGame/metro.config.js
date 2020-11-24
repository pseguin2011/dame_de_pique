const path = require('path');
const extraNodeModules = {
  'common': path.resolve(__dirname + '/../common'),
};
const watchFolders = [
  path.resolve(__dirname + '/../common')
];

module.exports = {
  transformer: {
    getTransformOptions: async () => ({
      transform: {
        experimentalImportSupport: false,
        inlineRequires: false,
      },
    }),
  }, 
  resolver: {
    extraNodeModules
  },
  watchFolders,
};
