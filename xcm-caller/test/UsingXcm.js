const { expect } = require("chai")
const { ethers } = require("hardhat")

describe("UsingXcm", function () {
    let xcm, owner, addr1, addr2

    beforeEach(async () => {
        ;[owner, addr1, addr2] = await ethers.getSigners()

        const usingXcm = await ethers.getContractFactory("UsingXcm")
        xcm = await usingXcm.deploy()
        await xcm.waitForDeployment()
    })
})
