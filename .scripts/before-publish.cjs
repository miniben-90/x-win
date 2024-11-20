const path = require('node:path');
const fs = require('node:fs');

const { TAG } = process.env;

function raplceCargoVersion(cargoToml, version) {
  return cargoToml.replace(/version\s*=\s*".*?"/, `version = "${version}"`);
}

if (TAG && TAG.startsWith('napi-')) {
  const tag = TAG.replace('napi-', '');
  const version = tag.startsWith('v') ? tag.substring(1) : tag;
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
} else {
  console.error('TAG env not found!');
  process.exit(1);
}

/** Exist process with success */
process.exit(0);