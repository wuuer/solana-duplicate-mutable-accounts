import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { expect } from "chai"
import { DuplicateMutableAccounts } from "../target/types/duplicate_mutable_accounts"

describe("duplicate-mutable-accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .DuplicateMutableAccounts as Program<DuplicateMutableAccounts>

  const playerOne = anchor.web3.Keypair.generate()
  const playerTwo = anchor.web3.Keypair.generate()

  it("Initialized Player One", async () => {
    await program.methods
      .initialize()
      .accounts({
        newPlayer: playerOne.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([playerOne])
      .rpc()
  })

  it("Initialized Player Two", async () => {
    await program.methods
      .initialize()
      .accounts({
        newPlayer: playerTwo.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([playerTwo])
      .rpc()
  })

  it("Invoke insecure instruction, expect wrong `games_played` count", async () => {
    await program.methods
      .rockPaperScissorsShootInsecure({ rock: {} }, { scissors: {} })
      .accounts({
        playerOne: playerOne.publicKey,
        playerTwo: playerOne.publicKey,
      })
      .rpc()

    const p1 = await program.account.playerState.fetch(playerOne.publicKey)

    expect(p1.choice.Scissors).to.equal({ scissors: {} })
  })
})
