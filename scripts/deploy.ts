/**

http://localhost:1317/ibc/core/channel/v1/channels?pagination.limit=1
http://localhost:1317/cosmos/bank/v1beta1/denoms_metadata
http://localhost:1316/cosmos/bank/v1beta1/balances/

*/

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import { Secp256k1HdWallet } from "@cosmjs/amino";
import "bun";

const TEST_MNEMONIC =
  "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass";

let adminClient = await Secp256k1HdWallet.fromMnemonic(TEST_MNEMONIC, {
  prefix: "neutron",
});

let signer = await SigningCosmWasmClient.connectWithSigner(
  "http://localhost:26657/",
  adminClient,
  {
    gasPrice: GasPrice.fromString("25untrn"),
  },
);

let adminAddress = (await adminClient.getAccounts())[0].address;
let BANK_CODE_ID: number;
let addr = "neutron1nyuryl5u5z04dx4zsqgvsuw7fe8gl2f77yufynauuhklnnmnjncqcls0tj";
let aw = "cosmos1dk4x42h886dfnrkzeupqxgwukthrwaneu6ejrswgx25lemx6f2kqn4zmrh";
if (false) {
  let bank = new Uint8Array(
    await Bun.file(
      "../contracts/target/wasm32-unknown-unknown/release/emporion_core.wasm",
    ).arrayBuffer(),
  );

  ({ codeId: BANK_CODE_ID } = await signer.upload(adminAddress, bank, "auto"));

  console.log(BANK_CODE_ID);

  let res = await signer.instantiate(
    adminAddress,
    BANK_CODE_ID,
    {},
    "BANK",
    "auto",
    {
      funds: [
        {
          denom: "untrn",
          amount: "100000000",
        },
      ],
    },
  );
  addr = res.contractAddress;
  console.log(addr);
} else {
  BANK_CODE_ID = 20;
}

let height = (
  await (
    await fetch(
      "http://localhost:1316/cosmos/base/tendermint/v1beta1/blocks/latest",
    )
  ).json()
).block.header.height;
console.log(height);

// let r = await signer.queryContractSmart(addr, "errors");

const r = await signer.execute(
  (await adminClient.getAccounts())[0].address,
  addr,
  "deposit",
  "auto",
);

const x = (_, v) => (typeof v === "bigint" ? v.toString() : v);

console.log(r);
