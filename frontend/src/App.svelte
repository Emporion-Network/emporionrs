<script>
    import Counter from "./lib/Counter.svelte";
    import Route from "./lib/Route.svelte";
    import {getLocation} from "./stores/location.svelte";
    let location = getLocation();
</script>

<Route path="/">
  <Counter path="/two"></Counter>
</Route>

<Route path="/two">
  <Counter path="/"></Counter>
</Route>

