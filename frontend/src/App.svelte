<script>
    import Counter from "./lib/Counter.svelte";
    import NavBar from "./lib/NavBar.svelte";
    import Route from "./lib/Route.svelte";
    import Bridge from "./routes/Bridge/Bridge.svelte";
    import Chat from "./routes/Chat/Chat.svelte";
    import CreateProduct from "./routes/CreateProduct/CreateProduct.svelte";
    import Store from "./routes/Store/Store.svelte";
    import { getLocation } from "./stores/location.svelte";
    let location = getLocation();
</script>

<NavBar></NavBar>
<Route path="/">
    <Counter path="/transfer"></Counter>
</Route>

<CreateProduct/>
<Chat />
<Bridge />
<Store />
