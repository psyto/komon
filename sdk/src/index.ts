// Client
export { MarketEngineClient } from "./client";

// PDA Utilities
export {
  MARKET_ENGINE_PROGRAM_ID,
  findMarketConfigPda,
  findMarketPda,
  findMarketVaultPda,
  findYesMintPda,
  findNoMintPda,
  findUserPositionPda,
  deriveMarketPdas,
} from "./pda";

// Types
export type {
  MarketConfig,
  Market,
  UserPosition,
  InitializeParams,
  CreateMarketParams,
  StakeParams,
  UnstakeParams,
  MarketCreatedEvent,
  StakePlacedEvent,
  StakeWithdrawnEvent,
  MarketSettledEvent,
  RewardsClaimedEvent,
  TransactionResult,
  CreateMarketResult,
  StakeResult,
} from "./types";

export { Outcome } from "./types";

// Re-export commonly used dependencies
export { BN } from "bn.js";
export { PublicKey } from "@solana/web3.js";
