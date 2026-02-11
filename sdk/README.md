# @komon/market-engine

TypeScript SDK for Komon Market Engine - Generic prediction markets on Solana.

## Installation

```bash
npm install @komon/market-engine
# or
yarn add @komon/market-engine
```

## Quick Start

```typescript
import { MarketEngineClient } from "@komon/market-engine";
import { AnchorProvider } from "@coral-xyz/anchor";

const provider = AnchorProvider.env();
const client = new MarketEngineClient(provider);
```

## Features

### Read Markets

```typescript
// Get global config
const config = await client.getConfig();
console.log(`Total markets: ${config.marketCount}`);

// Get a specific market
const market = await client.getMarket(subject, marketId);
console.log(`Liquidity: ${market.liquidity}`);
console.log(`YES supply: ${market.yesSupply}`);
console.log(`Settled: ${market.isSettled}`);

// Get a user's position
const position = await client.getUserPosition(marketPda, userPubkey);
```

### Create Markets

```typescript
const result = await client.createMarket(
  { description: "Will X happen?", analysis: "Analysis of the question" },
  subject,  // subject public key
  creator   // creator public key
);
console.log(`Market: ${result.marketAddress}`);
console.log(`ID: ${result.marketId}`);
```

### Stake on Outcomes

```typescript
import { Outcome } from "@komon/market-engine";
import { BN } from "bn.js";

// Stake on YES
await client.stake(
  { amount: new BN(1000000), outcome: Outcome.Yes },
  subject,
  marketId,
  stakeMint,
  staker
);

// Stake on NO
await client.stake(
  { amount: new BN(500000), outcome: Outcome.No },
  subject,
  marketId,
  stakeMint,
  staker
);
```

### Settlement and Rewards

```typescript
// Settle market (authority only)
await client.settleMarket(subject, marketId, Outcome.Yes, authority);

// Claim rewards
await client.claimRewards(subject, marketId, staker, burnTreasury);
```

### Utility Functions

```typescript
// Get implied probability from supply
const prob = client.getImpliedProbability(market);
console.log(`YES: ${(prob.yes * 100).toFixed(1)}%`);
console.log(`NO: ${(prob.no * 100).toFixed(1)}%`);

// Calculate expected payout
const payout = client.calculatePayout(market, winningBalance, burnRateBps, burnEnabled);
console.log(`Net payout: ${payout.netPayout}`);
```

## PDA Utilities

```typescript
import {
  MARKET_ENGINE_PROGRAM_ID,
  findMarketConfigPda,
  findMarketPda,
  findMarketVaultPda,
  findYesMintPda,
  findNoMintPda,
  findUserPositionPda,
  deriveMarketPdas,
} from "@komon/market-engine";

// Derive all PDAs for a market at once
const pdas = deriveMarketPdas(subject, marketId);
console.log(pdas.market, pdas.vault, pdas.yesMint, pdas.noMint);
```

## Types

```typescript
import type {
  MarketConfig,
  Market,
  UserPosition,
  CreateMarketParams,
  StakeParams,
  UnstakeParams,
  TransactionResult,
  CreateMarketResult,
} from "@komon/market-engine";
```

## License

MIT
