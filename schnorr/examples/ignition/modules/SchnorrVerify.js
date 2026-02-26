// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules")
const SchnorrVerifyModule = buildModule("SchnorrVerifyMod", (m) => {
    const schnorr = m.contract("SchnorrVerify", [])

    return { schnorr }
})

module.exports = SchnorrVerifyModule
