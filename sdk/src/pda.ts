import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

export const MARKET_ENGINE_PROGRAM_ID = new PublicKey(
  "5tFcB5fM3f19PB2CCqjBKvaNFqXWKgNjMALWknG291Qq"
);

// PDA Seeds
const SEEDS = {
  marketConfig: Buffer.from("market_config"),
  market: Buffer.from("market"),
  marketVault: Buffer.from("market_vault"),
  yesMint: Buffer.from("yes_mint"),
  noMint: Buffer.from("no_mint"),
  userPosition: Buffer.from("user_position"),
} as const;

export function findMarketConfigPda(
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.marketConfig],
    programId
  );
}

export function findMarketPda(
  subject: PublicKey,
  marketId: BN | number,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  const id = typeof marketId === "number" ? new BN(marketId) : marketId;
  return PublicKey.findProgramAddressSync(
    [SEEDS.market, subject.toBuffer(), id.toArrayLike(Buffer, "le", 8)],
    programId
  );
}

export function findMarketVaultPda(
  market: PublicKey,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.marketVault, market.toBuffer()],
    programId
  );
}

export function findYesMintPda(
  market: PublicKey,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.yesMint, market.toBuffer()],
    programId
  );
}

export function findNoMintPda(
  market: PublicKey,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.noMint, market.toBuffer()],
    programId
  );
}

export function findUserPositionPda(
  market: PublicKey,
  user: PublicKey,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.userPosition, market.toBuffer(), user.toBuffer()],
    programId
  );
}

/**
 * Derive all PDAs for a given market
 */
export function deriveMarketPdas(
  subject: PublicKey,
  marketId: BN | number,
  programId: PublicKey = MARKET_ENGINE_PROGRAM_ID
) {
  const [market, marketBump] = findMarketPda(subject, marketId, programId);
  const [vault, vaultBump] = findMarketVaultPda(market, programId);
  const [yesMint, yesMintBump] = findYesMintPda(market, programId);
  const [noMint, noMintBump] = findNoMintPda(market, programId);

  return {
    market,
    marketBump,
    vault,
    vaultBump,
    yesMint,
    yesMintBump,
    noMint,
    noMintBump,
  };
}
