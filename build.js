// eslint-disable-next-line
const util = require('util');
// eslint-disable-next-line
const exec = util.promisify(require('child_process').exec);
// eslint-disable-next-line
const { blue, green, yellow } = require('colors/safe');

const log = console.log;

(async () => {
  const mode = process.argv?.[2]?.trim?.()?.toLowerCase?.();

  if (mode !== `dev` && mode !== `prod`) {
    console.error(`Invalid mode -- must be 'dev' or 'prod', you have ${mode}`);
    return 1;
  }
  const capitalized = mode.toUpperCase();

  async function compileAws() {
    log('');
    log('');
    log(yellow(`AWS Stack ${capitalized} build . . .`));
    log('');
    log('');
    const aws = await exec(`yarn build:aws:${mode}`);
    console.log(`AWS stdout:\n${aws.stdout}`);
    console.log(`AWS stderr:\n${aws.stderr}`);
    log('');
    log('');
    log(green(`AWS Stack ${capitalized} build completed`));
    log('');
    log('');
  }

  async function compileUi() {
    log('');
    log('');
    log(yellow(`UI ${capitalized} build . . .`));
    log('');
    log('');
    const ui = await exec(`yarn build:ui:${mode}`);
    console.log(`UI stdout:\n${ui.stdout}`);
    console.log(`UI stderr:\n${ui.stderr}`);
    log('');
    log('');
    log(green(`UI ${capitalized} build completed`));
    log('');
    log('');
  }

  log('');
  log('');
  log(blue(`Beginning ${capitalized} build for AWS . . .`));
  Promise.all([compileAws(), compileUi()]);
})();
