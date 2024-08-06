const path = require('node:path');
const fs = require('node:fs');
const { argv } = require('node:process');

function raplceCargoVersion(cargoToml, version) {
  return cargoToml.replace(/version\s*=\s*".*?"/, `version = "${version}"`);
}

if (process.env.TAG) {
  const version = process.env.TAG.startsWith('v') ? process.env.TAG.substring(1) : process.env.TAG;

  if (argv && argv.findIndex(v => v === 'x-win-rs') !== -1) {
    console.log('[BEFORE-PUBLISH]', 'Update cargo.toml version to', version);

    const cargoTomlPath = path.join(process.cwd(), process.cwd().endsWith('x-win-rs') ? '' : 'x-win-rs', 'Cargo.toml');
    const cargoToml = fs.readFileSync(cargoTomlPath, { encoding: 'utf8' });

    console.log('[BEFORE-PUBLISH]', 'Updating cargo.toml');

    const newCargoToml = raplceCargoVersion(cargoToml, version);
    fs.writeFileSync(cargoTomlPath, newCargoToml);

    console.log('[BEFORE-PUBLISH]', 'Cargo.toml updated.');
    console.log('[BEFORE-PUBLISH]', 'Finished.');
  } else {
    console.log('[BEFORE-PUBLISH]', 'Update package.json and cargo.toml version to', version);

    const packageJsonPath = path.join(process.cwd(), 'package.json');
    const cargoTomlPath = path.join(process.cwd(), 'Cargo.toml');

    if (!fs.existsSync(packageJsonPath)) {
      console.error('package.json file not found!');
      process.exit(1);
    }

    if (!fs.existsSync(cargoTomlPath)) {
      console.error('package.json file not found!');
      process.exit(1);
    }

    const packageJson = require(packageJsonPath);
    console.log('[BEFORE-PUBLISH]', 'Updating package.json...');

    packageJson.version = version;
    fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, undefined, 2));

    console.log('[BEFORE-PUBLISH]', 'package.json updated.');

    const cargoToml = fs.readFileSync(cargoTomlPath, { encoding: 'utf8' });

    console.log('[BEFORE-PUBLISH]', 'Updating cargo.toml');

    const newCargoToml = raplceCargoVersion(cargoToml, version);
    fs.writeFileSync(cargoTomlPath, newCargoToml);

    console.log('[BEFORE-PUBLISH]', 'Cargo.toml updated.');
    console.log('[BEFORE-PUBLISH]', 'Finished.');
  }
} else {
  console.error('TAG env not found!');
  process.exit(1);
}

/** Exist process with success */
process.exit(0);