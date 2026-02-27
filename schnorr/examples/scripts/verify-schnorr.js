const hre = require("hardhat");

const PAS_UNITS = 10_000_000_000n;
const beneficiary =
  "0x814858e454d32daa4140447333ae0ff3c34511484669da6bd64c0caad798e08d";

const SIGNATURE_INPUT =
  "0x1b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078fb9c1fd76a80ce985843638b85b9c407ed76117be29a0dacb72257785b4cd661e6b88fdd3b0045e88503db4f81bbe56df3f552cd56d29a87767da7451e3b33561b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4";

async function main() {
  const contractAddress = "0xB1D1F00B6Cd1e148410B50F319BcbFa65ea367b6";
  const schnoorr = await hre.ethers.getContractFactory("SchnorrVerify");
  const schnorrContract = schnoorr.attach(contractAddress);

  const tx1 = await schnorrContract.verifySchnorrSignature(SIGNATURE_INPUT);
  console.log({ tx1 });
  console.dir({ tx1 });

  const hashInput = "0x48656c6c6f2c207768876c64";
  const tx = await schnorrContract.callHashBlake256(hashInput);
//   const receipt = await tx.wait();
  console.log({ tx });
  console.dir({ tx });
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
