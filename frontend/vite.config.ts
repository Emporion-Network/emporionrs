import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { paraglide } from "@inlang/paraglide-vite"


// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte(), 
    paraglide({
      project: "./project.inlang", //Path to your inlang project
      outdir: "./src/paraglide", //Where you want the generated files to be placed
    }),
  ],
})
