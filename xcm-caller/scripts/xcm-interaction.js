const hre = require("hardhat");

const PAS_UNITS = 10_000_000_000n;
const beneficiary = "0x814858e454d32daa4140447333ae0ff3c34511484669da6bd64c0caad798e08d";

async function main() {
    const contractAddress = "0xB1D1F00B6Cd1e148410B50F319BcbFa65ea367b6";
    const usingXcm = await hre.ethers.getContractFactory("UsingXcm");
    const xcmContract = usingXcm.attach(contractAddress);
    const message = "0x050c00040100000b002c7d8c3a0330010000070010a5d4e83101010051140100000401000002286bee000400010204040d01020400010100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    
    const tx = await xcmContract.teleportWithMessage(
       message
    );
    const receipt = await tx.wait();
    console.log({ receipt });
    console.dir({ receipt });
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });