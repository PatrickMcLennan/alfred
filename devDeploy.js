// eslint-disable-next-line
const util = require('util');
// eslint-disable-next-line
const exec = util.promisify(require('child_process').exec);
// eslint-disable-next-line
const { blue, green, yellow } = require('colors/safe');

const log = console.log;

(async () => {
  const mode = process.argv?.[2]?.trim?.()?.toLowerCase?.();

  if (mode !== `dev` || mode !== `prod`) {
    console.error(`Invalid mode -- must be 'dev' or 'prod', you have ${mode}`);
    return 1;
  }

  const capitalized = mode.toUpperCase();
  log(blue(`Beginning ${capitalized} build . . .`));
  log('');
  log('');
  log(yellow(`AWS Stack ${capitalized} build . . .`));
  const aws = await exec(`yarn build:aws:${mode}`);
  console.log('aws stdout:', aws.stdout);
  console.log('aws stderr:', aws.stderr);
  log(green(`AWS Stack ${capitalized} build completed`));
  log('');
  log('');
  log(yellow(`UI ${capitalized} build . . .`));
  const ui = await exec(`yarn build:ui:${mode}`);
  console.log('ui stdout:', ui.stdout);
  console.log('ui stderr:', ui.stderr);
  log(green(`UI ${capitalized} build completed`));
})();
