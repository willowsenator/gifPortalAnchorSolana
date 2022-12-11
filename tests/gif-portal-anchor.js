const anchor = require('@project-serum/anchor')
const main = async() =>{
  console.log('Starting tests')
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.GifPortalAnchor;
  const baseAccount = anchor.web3.Keypair.generate();
  const tx = await program.rpc.startStuffOff(
      {
        accounts:{
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers:[baseAccount]
      }
  )
  console.log('Transaction signature: ', tx)
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log("GIF count: ", account.totalGifs.toString())

  await program.rpc.addGif("https://media0.giphy.com/media/3o7TKrEzvLbsVAud8I/giphy.gif",{
    accounts:{
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log("GIF count: ", account.totalGifs.toString())
  console.log("GIF list: ", account.gifList)
}

const runMain = async() =>{
  try{
    await main()
    process.exit(0)
  }
  catch(err){
    console.log(err)
    process.exit(1)
  }
}

runMain();
