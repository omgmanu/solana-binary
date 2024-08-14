import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { BtcBinaryOptions } from "../target/types/btc_binary_options";
import { before, describe, test, it } from "node:test";
import { BN } from "bn.js";

describe("btc_binary_options", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BtcBinaryOptions as Program<BtcBinaryOptions>;

  it("Can start a game", async () => {
    const game = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    const timeframe = new BN(60); // 1 minute
    const betAmount = new BN(1_000_000_000); // 1 SOL
    const prediction = true; // Predicting price will go up

    const transactionSignature = await program.methods
      .startGame(timeframe, betAmount, prediction)
      .accounts({
        game: game.publicKey,
        player: player.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([game])
      .rpc();
      
    console.log("transactionSignature", transactionSignature);

    const gameAccount = await program.account.game.fetch(game.publicKey);

    assert.equal(gameAccount.player.toBase58(), player.publicKey.toBase58());
    assert.equal(gameAccount.betAmount.toNumber(), betAmount.toNumber());
    assert.equal(gameAccount.prediction, prediction);
    assert.equal(gameAccount.isSettled, false);
  });
});