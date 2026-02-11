import { describe, it, expect } from "vitest";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import {
  MARKET_ENGINE_PROGRAM_ID,
  findMarketConfigPda,
  findMarketPda,
  findMarketVaultPda,
  findYesMintPda,
  findNoMintPda,
  findUserPositionPda,
  deriveMarketPdas,
} from "./pda";

describe("PDA derivation", () => {
  const subject = new PublicKey("11111111111111111111111111111112");
  const user = new PublicKey("11111111111111111111111111111113");

  it("derives market config PDA deterministically", () => {
    const [pda1, bump1] = findMarketConfigPda();
    const [pda2, bump2] = findMarketConfigPda();
    expect(pda1.equals(pda2)).toBe(true);
    expect(bump1).toBe(bump2);
  });

  it("derives market PDA from subject and id", () => {
    const [pda0] = findMarketPda(subject, 0);
    const [pda1] = findMarketPda(subject, 1);
    expect(pda0.equals(pda1)).toBe(false);

    // BN and number should produce same result
    const [pdaBn] = findMarketPda(subject, new BN(1));
    expect(pda1.equals(pdaBn)).toBe(true);
  });

  it("derives market vault PDA from market", () => {
    const [market] = findMarketPda(subject, 0);
    const [vault, bump] = findMarketVaultPda(market);
    expect(vault).toBeInstanceOf(PublicKey);
    expect(bump).toBeGreaterThanOrEqual(0);
    expect(bump).toBeLessThanOrEqual(255);
  });

  it("derives YES and NO mint PDAs from market", () => {
    const [market] = findMarketPda(subject, 0);
    const [yesMint] = findYesMintPda(market);
    const [noMint] = findNoMintPda(market);
    expect(yesMint.equals(noMint)).toBe(false);
  });

  it("derives user position PDA from market and user", () => {
    const [market] = findMarketPda(subject, 0);
    const [position, bump] = findUserPositionPda(market, user);
    expect(position).toBeInstanceOf(PublicKey);
    expect(bump).toBeGreaterThanOrEqual(0);
  });

  it("deriveMarketPdas returns all PDAs for a market", () => {
    const pdas = deriveMarketPdas(subject, 0);
    expect(pdas.market).toBeInstanceOf(PublicKey);
    expect(pdas.vault).toBeInstanceOf(PublicKey);
    expect(pdas.yesMint).toBeInstanceOf(PublicKey);
    expect(pdas.noMint).toBeInstanceOf(PublicKey);

    // All addresses should be unique
    const addresses = [pdas.market, pdas.vault, pdas.yesMint, pdas.noMint];
    const unique = new Set(addresses.map((a) => a.toString()));
    expect(unique.size).toBe(4);
  });

  it("accepts custom program ID", () => {
    const customId = new PublicKey("11111111111111111111111111111114");
    const [defaultPda] = findMarketConfigPda();
    const [customPda] = findMarketConfigPda(customId);
    expect(defaultPda.equals(customPda)).toBe(false);
  });
});
