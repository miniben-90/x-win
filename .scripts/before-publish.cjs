const path = require('node:path');
const fs = require('node:fs');

console.log('[BEFORE-PUBLISH]', 'Update package.json and cargo.toml version...');

const packageJsonPath = path.join(process.cwd(), 'package.json');
const cargoTomlPath = path.join(process.cwd(), 'Cargo.toml');

const packageJson = require(packageJsonPath);

if (packageJson && process.env.TAG) {
  const version = process.env.TAG.startsWith('v') ? process.env.TAG.substring(1) : process.env.TAG;
  console.log('[BEFORE-PUBLISH]', 'Update package.json version to', version);
  packageJson.version = version;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, undefined, 2));

  const cargoToml = fs.readFileSync(cargoTomlPath, { encoding: 'utf8' });
  console.log('[BEFORE-PUBLISH]', 'Update cargo.toml version to', version);
  const newCargoToml = cargoToml.replace(/version\s*=\s*".*?"/, `version = "${version}"`);
  fs.writeFileSync(cargoTomlPath, newCargoToml);
} else {
  console.error('package or TAG env not found!');
  process.exit(1);
}

console.log('[BEFORE-PUBLISH]', 'Finished.');