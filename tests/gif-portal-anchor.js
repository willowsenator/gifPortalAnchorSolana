const anchor = require('@project-serum/anchor')
const main = async() =>{
  console.log('Starting tests')
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.GifPortalAnchor;
  const tx = await program.rpc.startStuffOff()
  console.log('Transaction signature: ', tx)
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
