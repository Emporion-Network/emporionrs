import { API } from "./api.svelte";

export let api:API = new API('http://localhost:3000');

$effect.root(() => {
    api.auth();
    // api.updated((nu)=>{
    //     api.state = nu;
    // })
})