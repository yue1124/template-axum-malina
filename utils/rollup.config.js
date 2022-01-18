import resolve from '@rollup/plugin-node-resolve';
import css from 'rollup-plugin-css-only';
import { terser } from "rollup-plugin-terser";
import malina from 'malinajs/malina-rollup'
import malinaSass from 'malinajs/plugins/sass'

const DEV = process.env.CARGO_MALINA_DEV === "true" ? true : false;
const cssInJS = false;

export default {
    input: 'src/main.js',
    output: {
        file: '../static/bundle.js',
        format: 'iife',
    },
    plugins: [
        malina({
            hideLabel: !DEV,
            css: cssInJS,
            plugins: [malinaSass()]
        }),
        resolve(),
        !cssInJS && css({ output: 'bundle.css' }),
        !DEV && terser()
    ],
    watch: {
        clearScreen: false
    }
}