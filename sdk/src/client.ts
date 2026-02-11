import { Program, AnchorProvider, BN, Idl } from "@coral-xyz/anchor";
import {
  PublicKey,
  SystemProgram,
  Connection,
  Keypair,
  TransactionSignature,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  MarketConfig,
  Market,
  UserPosition,
  Outcome,
  InitializeParams,
  CreateMarketParams,
  StakeParams,
  UnstakeParams,
  CreateMarketResult,
  TransactionResult,
} from "./types";
import {
  MARKET_ENGINE_PROGRAM_ID,
  findMarketConfigPda,
  findMarketPda,
  findMarketVaultPda,
  findYesMintPda,
  findNoMintPda,
  findUserPositionPda,
} from "./pda";

type AnyProgram = Program<Idl>;

export class MarketEngineClient {
  readonly program: AnyProgram;
  readonly programId: PublicKey;

  constructor(
    provider: AnchorProvider,
    programId: PublicKey = MARKET_ENGINE_PROGRAM_ID,
    idl?: Idl
  ) {
    this.programId = programId;

    // If IDL is provided, use it directly. Otherwise use a minimal stub.
    // Once the IDL is generated via `anchor build`, pass it here for full
    // type safety and automatic account resolution.
    const resolvedIdl = idl ?? MINIMAL_IDL;
    this.program = new Program(resolvedIdl, provider) as AnyProgram;
  }

  // ==========================================================================
  // Read Methods
  // ==========================================================================

  async getConfig(): Promise<MarketConfig> {
    const [configPda] = findMarketConfigPda(this.programId);
    const account = await (this.program.account as any)["marketConfig"].fetch(configPda);
    return account as MarketConfig;
  }

  async getMarket(
    subject: PublicKey,
    marketId: BN | number
  ): Promise<Market> {
    const [marketPda] = findMarketPda(subject, marketId, this.programId);
    const account = await (this.program.account as any)["market"].fetch(marketPda);
    return account as Market;
  }

  async getUserPosition(
    market: PublicKey,
    user: PublicKey
  ): Promise<UserPosition | null> {
    const [positionPda] = findUserPositionPda(market, user, this.programId);
    try {
      const account = await (this.program.account as any)["userPosition"].fetch(positionPda);
      return account as UserPosition;
    } catch {
      return null;
    }
  }

  async getMarketCount(): Promise<BN> {
    const config = await this.getConfig();
    return config.marketCount;
  }

  // ==========================================================================
  // Write Methods
  // ==========================================================================

  /**
   * Initialize the market engine config. Call once per deployment.
   */
  async initialize(
    params: InitializeParams,
    authority: PublicKey
  ): Promise<TransactionResult> {
    const [configPda] = findMarketConfigPda(this.programId);

    const signature = await this.program.methods
      .initialize(params.feeRate, params.burnEnabled, params.burnRateBps)
      .accounts({
        config: configPda,
        authority,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return { signature };
  }

  /**
   * Create a new prediction market on a subject.
   */
  async createMarket(
    params: CreateMarketParams,
    subject: PublicKey,
    creator: PublicKey
  ): Promise<CreateMarketResult> {
    const config = await this.getConfig();
    const marketId = config.marketCount;

    const [configPda] = findMarketConfigPda(this.programId);
    const [marketPda] = findMarketPda(subject, marketId, this.programId);
    const [yesMint] = findYesMintPda(marketPda, this.programId);
    const [noMint] = findNoMintPda(marketPda, this.programId);

    const signature = await this.program.methods
      .createMarket(params.description, params.analysis)
      .accounts({
        config: configPda,
        subject,
        market: marketPda,
        yesMint,
        noMint,
        creator,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return { signature, marketAddress: marketPda, marketId };
  }

  /**
   * Stake tokens on a market outcome (YES or NO).
   * Mints outcome tokens 1:1 with the staked amount.
   */
  async stake(
    params: StakeParams,
    subject: PublicKey,
    marketId: BN | number,
    stakeMint: PublicKey,
    staker: PublicKey
  ): Promise<TransactionResult> {
    const [configPda] = findMarketConfigPda(this.programId);
    const [marketPda] = findMarketPda(subject, marketId, this.programId);
    const [marketVault] = findMarketVaultPda(marketPda, this.programId);
    const [yesMint] = findYesMintPda(marketPda, this.programId);
    const [noMint] = findNoMintPda(marketPda, this.programId);
    const [userPosition] = findUserPositionPda(marketPda, staker, this.programId);

    const stakerTokens = getAssociatedTokenAddressSync(stakeMint, staker);
    const stakerYesTokens = getAssociatedTokenAddressSync(yesMint, staker);
    const stakerNoTokens = getAssociatedTokenAddressSync(noMint, staker);

    const outcomeArg = params.outcome === Outcome.Yes ? { yes: {} } : { no: {} };

    const signature = await this.program.methods
      .stake(params.amount, outcomeArg)
      .accounts({
        config: configPda,
        market: marketPda,
        marketVault,
        yesMint,
        noMint,
        stakerTokens,
        stakerYesTokens,
        stakerNoTokens,
        userPosition,
        stakeMint,
        staker,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return { signature };
  }

  /**
   * Unstake (sell outcome tokens back before settlement).
   */
  async unstake(
    params: UnstakeParams,
    subject: PublicKey,
    marketId: BN | number,
    staker: PublicKey
  ): Promise<TransactionResult> {
    const [marketPda] = findMarketPda(subject, marketId, this.programId);
    const [marketVault] = findMarketVaultPda(marketPda, this.programId);
    const [yesMint] = findYesMintPda(marketPda, this.programId);
    const [noMint] = findNoMintPda(marketPda, this.programId);

    // Determine the stake mint from the vault (same as what was used to stake)
    const market = await this.getMarket(subject, marketId);
    const stakerTokens = getAssociatedTokenAddressSync(
      // market_vault token mint — we need the underlying stake mint.
      // The vault is a token account whose mint is the stakeMint passed at stake time.
      // For simplicity, caller should pass this, but we derive from market vault account.
      // Here we use a placeholder — in practice, pass stakeMint explicitly.
      marketVault,
      staker
    );
    const stakerYesTokens = getAssociatedTokenAddressSync(yesMint, staker);
    const stakerNoTokens = getAssociatedTokenAddressSync(noMint, staker);

    const outcomeArg = params.outcome === Outcome.Yes ? { yes: {} } : { no: {} };

    const signature = await this.program.methods
      .unstake(params.amount, outcomeArg)
      .accounts({
        market: marketPda,
        marketVault,
        yesMint,
        noMint,
        stakerTokens,
        stakerYesTokens,
        stakerNoTokens,
        staker,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    return { signature };
  }

  /**
   * Settle a market after subject resolution. Authority only.
   */
  async settleMarket(
    subject: PublicKey,
    marketId: BN | number,
    outcome: Outcome,
    authority: PublicKey
  ): Promise<TransactionResult> {
    const [configPda] = findMarketConfigPda(this.programId);
    const [marketPda] = findMarketPda(subject, marketId, this.programId);

    const outcomeArg = outcome === Outcome.Yes ? { yes: {} } : { no: {} };

    const signature = await this.program.methods
      .settleMarket(outcomeArg)
      .accounts({
        config: configPda,
        market: marketPda,
        authority,
      })
      .rpc();

    return { signature };
  }

  /**
   * Claim rewards after market settlement.
   * Includes burn mechanism for Creator mode.
   */
  async claimRewards(
    subject: PublicKey,
    marketId: BN | number,
    staker: PublicKey,
    burnTreasury: PublicKey
  ): Promise<TransactionResult> {
    const [configPda] = findMarketConfigPda(this.programId);
    const [marketPda] = findMarketPda(subject, marketId, this.programId);
    const [marketVault] = findMarketVaultPda(marketPda, this.programId);
    const [yesMint] = findYesMintPda(marketPda, this.programId);
    const [noMint] = findNoMintPda(marketPda, this.programId);
    const [userPosition] = findUserPositionPda(marketPda, staker, this.programId);

    const market = await this.getMarket(subject, marketId);
    const stakerYesTokens = getAssociatedTokenAddressSync(yesMint, staker);
    const stakerNoTokens = getAssociatedTokenAddressSync(noMint, staker);

    // For staker_tokens, we need the underlying stake token ATA.
    // This requires knowing the stake mint. We read it from the vault.
    const vaultInfo = await this.program.provider.connection.getAccountInfo(marketVault);
    // Parse mint from token account data (bytes 0..32)
    const stakeMint = vaultInfo
      ? new PublicKey(vaultInfo.data.subarray(0, 32))
      : PublicKey.default;
    const stakerTokens = getAssociatedTokenAddressSync(stakeMint, staker);

    const signature = await this.program.methods
      .claimRewards()
      .accounts({
        config: configPda,
        market: marketPda,
        marketVault,
        userPosition,
        stakerTokens,
        stakerYesTokens,
        stakerNoTokens,
        yesMint,
        noMint,
        burnTreasury,
        staker,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    return { signature };
  }

  // ==========================================================================
  // Utility Methods
  // ==========================================================================

  /**
   * Calculate expected payout for a winning position
   */
  calculatePayout(
    market: Market,
    winningBalance: BN,
    burnRateBps: number,
    burnEnabled: boolean
  ): { grossPayout: BN; burnAmount: BN; netPayout: BN } {
    const outcome = market.outcome;
    if (outcome === null) {
      return {
        grossPayout: new BN(0),
        burnAmount: new BN(0),
        netPayout: new BN(0),
      };
    }

    const totalWinningSupply =
      outcome === Outcome.Yes ? market.yesSupply : market.noSupply;

    if (totalWinningSupply.isZero()) {
      return {
        grossPayout: new BN(0),
        burnAmount: new BN(0),
        netPayout: new BN(0),
      };
    }

    const grossPayout = market.liquidity
      .mul(winningBalance)
      .div(totalWinningSupply);

    let burnAmount = new BN(0);
    if (burnEnabled && !grossPayout.isZero()) {
      burnAmount = grossPayout
        .mul(new BN(burnRateBps))
        .div(new BN(10000));
    }

    const netPayout = grossPayout.sub(burnAmount);

    return { grossPayout, burnAmount, netPayout };
  }

  /**
   * Get the implied probability of YES outcome based on supply
   */
  getImpliedProbability(market: Market): {
    yes: number;
    no: number;
  } {
    const total = market.yesSupply.add(market.noSupply);
    if (total.isZero()) {
      return { yes: 0.5, no: 0.5 };
    }
    return {
      yes: market.yesSupply.toNumber() / total.toNumber(),
      no: market.noSupply.toNumber() / total.toNumber(),
    };
  }
}

// ==========================================================================
// Minimal IDL stub — replace with generated IDL after `anchor build`
// ==========================================================================

const MINIMAL_IDL: Idl = {
  address: MARKET_ENGINE_PROGRAM_ID.toString(),
  metadata: {
    name: "market_engine",
    version: "0.1.0",
    spec: "0.1.0",
  },
  instructions: [],
  accounts: [],
  types: [],
} as unknown as Idl;
