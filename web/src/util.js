
import {v4 as uuidv4} from 'uuid';

export function getGuid() {
    return uuidv4();
}

export function blurOnKey(event) {
    const {code} = event;
    if (code === 'Enter' || code === 'Escape' || code === 'Tab') {
        event.target.blur();
    }
}

export function sortOnName(array) {
    array.sort((e1, e2) => 
        e1.name.toLowerCase().localeCompare(e2.name.toLowerCase())
    );

    return array;
}
    // In package.json:
    // "@rollup/plugin-node-resolve": "^7.1.1",
    // "eslint": "^6.8.0",
    // "eslint-plugin-import": "^2.20.2",
    // "eslint-plugin-prettier": "^3.1.2",
    // "eslint-plugin-svelte3": "^2.7.3",
    // "prettier": "^1.19.1",
    // "prettier-plugin-svelte": "^0.7.0",
    // "rollup": "^2.3.3",
    // "rollup-plugin-commonjs": "^10.1.0",
    // "rollup-plugin-livereload": "^1.1.0",
    // "rollup-plugin-svelte": "^5.2.1",
    // "rollup-plugin-terser": "^5.3.0"