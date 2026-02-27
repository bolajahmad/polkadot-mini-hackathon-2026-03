require("@nomicfoundation/hardhat-toolbox")
require("@parity/hardhat-polkadot")

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    solidity: "0.8.28",
    networks: {
        hardhat: {
            polkadot: {
                target: "evm",
            },
            nodeConfig: {
                nodeBinaryPath: "./bin/polkadot-parachain",
                rpcPort: 61315,
                dev: true,
            },
            adapterConfig: {
                adapterBinaryPath: "./bin/eth-rpc",
                dev: true,
            },
        },
        localNode: {
            polkadot: {
                target: "evm",
            },
            // This is for your ALREADY RUNNING Zombienet + eth-rpc
            url: "http://127.0.0.1:8545",
            chainId: 420420421,
            accounts: [vars.get("LOCAL_PRIVATE_KEY")],
        },
        // polkadotHubTestnet: {
        //     polkadot: {
        //         target: "evm",
        //     },
        //     url: "https://testnet-passet-hub-eth-rpc.polkadot.io",
        //     accounts: [vars.get("PRIVATE_KEY")],
        // },
    },
}
