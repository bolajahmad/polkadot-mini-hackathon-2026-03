const { ethers, network } = require("hardhat");

async function main() {
  // Get the signer from the private key in your config
  const [signer] = await ethers.getSigners();
  const address = await signer.getAddress();

  console.log(`\nNetwork: ${network.name}`);
  console.log(`Checking balance for Alice (ECDSA): ${address}`);

  // Fetch balance from the eth-rpc bridge
  const balance = await ethers.provider.getBalance("0xB1D1F00B6Cd1e148410B50F319BcbFa65ea367b6");

  console.log(`Balance: ${ethers.formatEther(balance)} WND`);

  if (balance === 0n) {
    console.log("\n⚠️  WARNING: Balance is 0! Deployment will fail.");
    console.log("To fix this:");
    console.log(`1. Open Polkadot-JS on Port 9848`);
    console.log(`2. Send WND from 'Alice' to the address above: ${address}`);
  } else {
    console.log("\n✅ Success! You have enough funds to deploy.");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
