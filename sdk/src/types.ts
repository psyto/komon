import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

// ============================================================================
// Enums
// ============================================================================

export enum Outcome {
  Yes = 0,
  No = 1,
}

// ============================================================================
// On-chain Account Types
// ============================================================================

export interface MarketConfig {
  authority: PublicKey;
  marketCount: BN;
  feeRate: number;
  burnEnabled: boolean;
  burnRateBps: number;
  totalVolume: BN;
  totalBurned: BN;
  bump: number;
}

export interface Market {
  id: BN;
  subject: PublicKey;
  creator: PublicKey;
  description: string;
  analysis: string;
  yesMint: PublicKey;
  noMint: PublicKey;
  yesSupply: BN;
  noSupply: BN;
  liquidity: BN;
  isSettled: boolean;
  outcome: Outcome | null;
  createdAt: BN;
  bump: number;
}

export interface UserPosition {
  user: PublicKey;
  market: PublicKey;
  claimed: boolean;
  bump: number;
}

// ============================================================================
// Instruction Params
// ============================================================================

export interface InitializeParams {
  feeRate: number;
  burnEnabled: boolean;
  burnRateBps: number;
}

export interface CreateMarketParams {
  description: string;
  analysis: string;
}

export interface StakeParams {
  amount: BN;
  outcome: Outcome;
}

export interface UnstakeParams {
  amount: BN;
  outcome: Outcome;
}

// ============================================================================
// Events
// ============================================================================

export interface MarketCreatedEvent {
  marketId: BN;
  subject: PublicKey;
  creator: PublicKey;
  description: string;
}

export interface StakePlacedEvent {
  marketId: BN;
  staker: PublicKey;
  amount: BN;
  outcome: Outcome;
  tokensMinted: BN;
}

export interface StakeWithdrawnEvent {
  marketId: BN;
  staker: PublicKey;
  amount: BN;
  outcome: Outcome;
}

export interface MarketSettledEvent {
  marketId: BN;
  outcome: Outcome;
  totalLiquidity: BN;
}

export interface RewardsClaimedEvent {
  marketId: BN;
  staker: PublicKey;
  grossPayout: BN;
  burnAmount: BN;
  netPayout: BN;
}

// ============================================================================
// Transaction Results
// ============================================================================

export interface TransactionResult {
  signature: string;
}

export interface CreateMarketResult extends TransactionResult {
  marketAddress: PublicKey;
  marketId: BN;
}

export interface StakeResult extends TransactionResult {
  tokensMinted: BN;
}
