// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules")

const BlsContractModule = buildModule("BlsContractModule", (m) => {
    const bls = m.contract("BlsContract", [])

    return { bls }
})

module.exports = BlsContractModule
