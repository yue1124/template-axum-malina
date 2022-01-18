const { accessSync, constants } = require('fs')
const path = require('path')

var node_exe_path = process.execPath;

try {
    let npm_cli_js_path = path.join(node_exe_path, '..', 'node_modules', 'npm', 'bin', 'npm-cli.js');
    accessSync(npm_cli_js_path, constants.R_OK);
    console.log(npm_cli_js_path);
} catch (err) {
    console.error('there is no npm-cli.js!');
}