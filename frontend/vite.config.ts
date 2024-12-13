import { defineConfig, PluginOption } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte';
import fs from "fs";
import path from "path";


function plugin():PluginOption {
    const virtualModuleId = 'virtual:messages'
    const resolvedVirtualModuleId = '\0' + virtualModuleId;

    return {
        name: 'my-plugin', // required, will show up in warnings and errors
        resolveId(id:string) {
            if (id === virtualModuleId) {
                return resolvedVirtualModuleId
            }
        },
        handleHotUpdate(ctx){
            // ctx.modules.filter((m)=> m.file != "messages.d.ts");
            let k = Object.keys(JSON.parse(fs.readFileSync("../messages/en.json", "utf-8"))).filter(e => e !== "$schema");
            let langs = JSON.parse(fs.readFileSync("../emporion.inlang/settings.json", "utf-8")).languageTags;
            fs.writeFileSync('messages.d.ts', `export type msg = "${k.join('"|"')}";
export const langs = ["${langs.join('","')}"] as const;
`);
            return ctx.modules.filter(e => !e.file?.endsWith("messages.d.ts"));
        }
    }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte(), plugin()],
})
