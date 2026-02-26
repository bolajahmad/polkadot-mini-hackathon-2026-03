const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules")

const XcmCallerModule = buildModule("UsingXcm", (m) => {
    const contract = m.contract("UsingXcm")
 
    return { contract }
})

module.exports = XcmCallerModule