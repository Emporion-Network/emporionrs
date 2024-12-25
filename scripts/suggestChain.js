await window.keplr.experimentalSuggestChain({
  chainId: "test-1",
  chainName: "neutron-test",
  rpc: "http://localhost:26657",
  rest: "http://localhost:1317",
  bip44: {
    coinType: 118,
  },
  bech32Config: {
    bech32PrefixAccAddr: "neutron",
    bech32PrefixAccPub: "neutron" + "pub",
    bech32PrefixValAddr: "neutron" + "valoper",
    bech32PrefixValPub: "neutron" + "valoperpub",
    bech32PrefixConsAddr: "neutron" + "valcons",
    bech32PrefixConsPub: "neutron" + "valconspub",
  },
  currencies: [
    {
      coinDenom: "Neutron",
      coinMinimalDenom: "untrn",
      coinDecimals: 6,
      coinGeckoId: "neutron-3",
    },
    {
      coinDenom: "CRZ",
      coinMinimalDenom:
        "factory/neutron1nyuryl5u5z04dx4zsqgvsuw7fe8gl2f77yufynauuhklnnmnjncqcls0tj/CRZS",
      coinDecimals: 6,
    },
  ],
  feeCurrencies: [
    {
      coinDenom: "Neutron",
      coinMinimalDenom: "untrn",
      coinDecimals: 6,
      coinGeckoId: "neutron-3",
      gasPriceStep: {
        low: 0.01,
        average: 0.025,
        high: 0.04,
      },
    },
  ],
  stakeCurrency: {
    coinDenom: "Neutron",
    coinMinimalDenom: "untrn",
    coinDecimals: 6,
    coinGeckoId: "neutron-3",
  },
});
