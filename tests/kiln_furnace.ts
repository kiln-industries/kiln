import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KilnFurnace } from "../target/types/kiln_furnace";
import { expect } from "chai";
import { SystemProgram, Keypair } from "@solana/web3.js";
import * as crypto from "crypto";

describe("KILN Furnace Integration Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.KilnFurnace as Program<KilnFurnace>;
  const authority = provider.wallet;

  let furnacePDA: anchor.web3.PublicKey;
  let furnaceBump: number;

  before(async () => {
    [furnacePDA, furnaceBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("furnace"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Ignition", () => {
    it("ignites a new furnace", async () => {
      const initialTemp = new anchor.BN(3000);

      const tx = await program.methods
        .ignite(initialTemp)
        .accounts({
          furnace: furnacePDA,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Ignition tx:", tx);

      const furnaceAccount = await program.account.furnace.fetch(furnacePDA);
      expect(furnaceAccount.currentTemp.toNumber()).to.equal(3000);
      expect(furnaceAccount.isActive).to.be.true;
    });
  });

  describe("Sintering", () => {
    it("sinters a data block successfully", async () => {
      const rawData = Buffer.from("test data for permanent storage");
      const dataHash = crypto.createHash("sha256").update(rawData).digest();
      const pressure = 120;

      // Find sintered block PDA
      const furnaceAccount = await program.account.furnace.fetch(furnacePDA);
      const [sinteredBlockPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("sintered"),
          furnacePDA.toBuffer(),
          furnaceAccount.totalSinteredBlocks.toBuffer("le", 8),
        ],
        program.programId
      );

      const tx = await program.methods
        .sinterBatch(Array.from(dataHash), pressure)
        .accounts({
          furnace: furnacePDA,
          sinteredBlock: sinteredBlockPDA,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Sinter tx:", tx);

      const blockAccount = await program.account.sinteredBlock.fetch(sinteredBlockPDA);
      expect(blockAccount.pressureApplied).to.equal(pressure);
    });

    it("rejects low pressure sintering", async () => {
      const dataHash = Array(32).fill(0);
      const lowPressure = 50; // Below minimum 90

      // Should fail with InsufficientPressure error
      try {
        await program.methods.sinterBatch(dataHash, lowPressure).rpc();
        expect.fail("Should have thrown");
      } catch (err) {
        expect(err.message).to.include("InsufficientPressure");
      }
    });
  });

  describe("Emergency Cooldown", () => {
    it("performs emergency cooldown", async () => {
      const tx = await program.methods
        .emergencyCooldown()
        .accounts({
          furnace: furnacePDA,
          authority: authority.publicKey,
        })
        .rpc();

      console.log("Cooldown tx:", tx);

      const furnaceAccount = await program.account.furnace.fetch(furnacePDA);
      expect(furnaceAccount.isActive).to.be.false;
      expect(furnaceAccount.currentTemp.toNumber()).to.equal(0);
    });
  });
});
