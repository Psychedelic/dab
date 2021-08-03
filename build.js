const { execSync } = require('child_process');

function buildWasm(pkg) {
    const underscoredName = pkg.replace(/-/g, '_');

    const buildCommand = [
        'cargo',
        'build',
        '--target',
        'wasm32-unknown-unknown',
        '--release',
        '--package',
        pkg,
    ];

    console.log(`Building ${underscoredName}.wasm`);
    execSync(buildCommand.join(' '));

    const optCommand = [
        'ic-cdk-optimizer',
        `target/wasm32-unknown-unknown/release/${underscoredName}.wasm`,
        '-o',
        `target/wasm32-unknown-unknown/release/${underscoredName}-opt.wasm`,
    ];

    console.log(`Running ic-cdk-optimizer on ${underscoredName}.wasm`);
    execSync(optCommand.join(' '));
}

buildWasm('address');
