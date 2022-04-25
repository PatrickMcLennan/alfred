// eslint-disable-next-line
const util = require('util');
// eslint-disable-next-line
const exec = util.promisify(require('child_process').exec);

(async () => {
  const aws = await exec('yarn build:aws:prod');
  const ui = await exec('yarn build:ui:prod');
  console.log('aws stdout:', aws.stdout);
  console.log('aws stderr:', aws.stderr);
  console.log('ui stdout:', ui.stdout);
  console.log('ui stderr:', ui.stderr);
})();
