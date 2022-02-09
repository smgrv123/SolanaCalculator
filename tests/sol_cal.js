const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.SolCal;

  const baseAcc = anchor.web3.Keypair.generate();

  const tx = await program.rpc.initialize({
    accounts: {
      baseAcc: baseAcc.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAcc],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.calc.fetch(baseAcc.publicKey);
  console.log("account", account);

  await program.rpc.modulas(new anchor.BN(1), new anchor.BN(2), {
    accounts: {
      baseAcc: baseAcc.publicKey,
    },
  });

  account = await program.account.calc.fetch(baseAcc.publicKey);
  console.log("account", account.result.eq(new anchor.BN(2)));
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
