import { Client } from "@neutron-org/client-ts";

let c = new Client({
  rpcURL: "https://neutron-rest.publicnode.com",
  apiURL: "https://neutron-rest.publicnode.com",
});

console.log((await c.NeutronFeerefunder.query.queryParams()).data.params);
