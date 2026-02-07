import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
} from '@solana/web3.js';
import { WalletContextState } from '@solana/wallet-adapter-react';

export const SOVEREIGN_PROGRAM_ID = new PublicKey('2UAZc1jj4QTSkgrC8U9d4a7EM9AQunxMvW5g7rX7Af9T');

export interface SovereignIdentity {
  owner: PublicKey;
  civicAuthority: PublicKey;
  tradingScore: number;
  civicScore: number;
  developerScore: number;
  infraScore: number;
  compositeScore: number;
  tier: number;
}

export interface KomonReputation {
  winRate: number;           // 0-100
  directionsWon: number;
  directionsProposed: number;
  currentStreak: number;
  level: number;
}

export function getIdentityPda(owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('identity'), owner.toBuffer()],
    SOVEREIGN_PROGRAM_ID
  );
}

export function getTierName(tier: number): string {
  switch (tier) {
    case 1: return 'Bronze';
    case 2: return 'Silver';
    case 3: return 'Gold';
    case 4: return 'Platinum';
    case 5: return 'Diamond';
    default: return 'None';
  }
}

export function getTierColor(tier: number): string {
  switch (tier) {
    case 1: return '#CD7F32'; // Bronze
    case 2: return '#C0C0C0'; // Silver
    case 3: return '#FFD700'; // Gold
    case 4: return '#E5E4E2'; // Platinum
    case 5: return '#B9F2FF'; // Diamond
    default: return '#666666';
  }
}

export function getTierEmoji(tier: number): string {
  switch (tier) {
    case 1: return '🥉';
    case 2: return '🥈';
    case 3: return '🥇';
    case 4: return '💎';
    case 5: return '👑';
    default: return '⭐';
  }
}

export async function fetchSovereignIdentity(
  connection: Connection,
  owner: PublicKey
): Promise<SovereignIdentity | null> {
  const [identityPda] = getIdentityPda(owner);

  const accountInfo = await connection.getAccountInfo(identityPda);
  if (!accountInfo) {
    return null;
  }

  const data = accountInfo.data;

  // Parse the account data
  // Layout: discriminator(8) + owner(32) + created_at(8) + authorities(4*32) + scores
  const ownerPubkey = new PublicKey(data.slice(8, 40));

  // Authorities start at offset 48 (after discriminator + owner + created_at)
  // civic_authority is the second authority (offset 80)
  const civicAuthority = new PublicKey(data.slice(80, 112));

  // Scores start at offset 176
  const tradingScore = data.readUInt16LE(176);
  const civicScore = data.readUInt16LE(178);
  const developerScore = data.readUInt16LE(180);
  const infraScore = data.readUInt16LE(182);
  const compositeScore = data.readUInt16LE(184);
  const tier = data.readUInt8(186);

  return {
    owner: ownerPubkey,
    civicAuthority,
    tradingScore,
    civicScore,
    developerScore,
    infraScore,
    compositeScore,
    tier,
  };
}

export function getStakeLimit(tier: number): number {
  switch (tier) {
    case 1: return 100;      // 100 USDC
    case 2: return 500;      // 500 USDC
    case 3: return 2000;     // 2,000 USDC
    case 4: return 10000;    // 10,000 USDC
    case 5: return Infinity; // Unlimited
    default: return 100;
  }
}

// Anchor instruction discriminator for create_identity
// sha256("global:create_identity")[0..8]
const CREATE_IDENTITY_DISCRIMINATOR = Buffer.from([12, 253, 209, 41, 176, 51, 195, 179]);

export async function createSovereignIdentity(
  connection: Connection,
  wallet: WalletContextState
): Promise<string> {
  if (!wallet.publicKey || !wallet.signTransaction) {
    throw new Error('Wallet not connected');
  }

  const [identityPda] = getIdentityPda(wallet.publicKey);

  // Check if identity already exists
  const existingAccount = await connection.getAccountInfo(identityPda);
  if (existingAccount) {
    throw new Error('SOVEREIGN identity already exists for this wallet');
  }

  // Build the create_identity instruction
  const createIdentityIx = new TransactionInstruction({
    programId: SOVEREIGN_PROGRAM_ID,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: identityPda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: CREATE_IDENTITY_DISCRIMINATOR,
  });

  // Create and send transaction
  const transaction = new Transaction().add(createIdentityIx);
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = wallet.publicKey;

  const signedTx = await wallet.signTransaction(transaction);
  const txId = await connection.sendRawTransaction(signedTx.serialize());

  // Wait for confirmation
  await connection.confirmTransaction({
    signature: txId,
    blockhash,
    lastValidBlockHeight,
  });

  return txId;
}

// Instruction discriminators
const SET_CIVIC_AUTHORITY_DISCRIMINATOR = Buffer.from([198, 212, 193, 82, 46, 159, 64, 206]);
const UPDATE_CIVIC_SCORE_DISCRIMINATOR = Buffer.from([62, 199, 170, 21, 48, 132, 219, 245]);

/**
 * Calculate civic score from Komon reputation metrics
 * Uses weighted formula matching SOVEREIGN's CivicScoreDetails
 */
export function calculateCivicScore(reputation: KomonReputation): number {
  // Weighted formula:
  // Win rate: 40%, Problems solved tier: 25%, Trust (level-based): 25%, Streak tier: 10%

  // Win rate component (0-10000)
  const winRateComponent = Math.round((reputation.winRate / 100) * 10000 * 0.4);

  // Solved tier based on directions won
  const solvedTier = (() => {
    if (reputation.directionsWon <= 2) return 2000;
    if (reputation.directionsWon <= 10) return 4000;
    if (reputation.directionsWon <= 50) return 6000;
    if (reputation.directionsWon <= 200) return 8000;
    return 10000;
  })();
  const solvedComponent = Math.round(solvedTier * 0.25);

  // Trust component based on level (proxy for community trust)
  const trustScore = Math.min(reputation.level * 500, 10000);
  const trustComponent = Math.round(trustScore * 0.25);

  // Streak tier
  const streakTier = (() => {
    if (reputation.currentStreak <= 2) return 2000;
    if (reputation.currentStreak <= 5) return 4000;
    if (reputation.currentStreak <= 10) return 6000;
    if (reputation.currentStreak <= 20) return 8000;
    return 10000;
  })();
  const streakComponent = Math.round(streakTier * 0.1);

  return Math.min(winRateComponent + solvedComponent + trustComponent + streakComponent, 10000);
}

/**
 * Set the civic authority on a SOVEREIGN identity
 * The owner must sign this transaction
 */
export async function setCivicAuthority(
  connection: Connection,
  wallet: WalletContextState,
  newAuthority: PublicKey
): Promise<string> {
  if (!wallet.publicKey || !wallet.signTransaction) {
    throw new Error('Wallet not connected');
  }

  const [identityPda] = getIdentityPda(wallet.publicKey);

  // Build instruction data: discriminator + new_authority pubkey
  const data = Buffer.concat([
    SET_CIVIC_AUTHORITY_DISCRIMINATOR,
    newAuthority.toBuffer(),
  ]);

  const setAuthorityIx = new TransactionInstruction({
    programId: SOVEREIGN_PROGRAM_ID,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: identityPda, isSigner: false, isWritable: true },
    ],
    data,
  });

  const transaction = new Transaction().add(setAuthorityIx);
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = wallet.publicKey;

  const signedTx = await wallet.signTransaction(transaction);
  const txId = await connection.sendRawTransaction(signedTx.serialize());

  await connection.confirmTransaction({
    signature: txId,
    blockhash,
    lastValidBlockHeight,
  });

  return txId;
}

/**
 * Update the civic score on a SOVEREIGN identity
 * The civic authority must sign this transaction
 */
export async function updateCivicScore(
  connection: Connection,
  wallet: WalletContextState,
  identityOwner: PublicKey,
  score: number
): Promise<string> {
  if (!wallet.publicKey || !wallet.signTransaction) {
    throw new Error('Wallet not connected');
  }

  if (score < 0 || score > 10000) {
    throw new Error('Score must be between 0 and 10000');
  }

  const [identityPda] = getIdentityPda(identityOwner);

  // Build instruction data: discriminator + score (u16 LE)
  const scoreBuffer = Buffer.alloc(2);
  scoreBuffer.writeUInt16LE(score, 0);

  const data = Buffer.concat([UPDATE_CIVIC_SCORE_DISCRIMINATOR, scoreBuffer]);

  const updateScoreIx = new TransactionInstruction({
    programId: SOVEREIGN_PROGRAM_ID,
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
      { pubkey: identityPda, isSigner: false, isWritable: true },
    ],
    data,
  });

  const transaction = new Transaction().add(updateScoreIx);
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = wallet.publicKey;

  const signedTx = await wallet.signTransaction(transaction);
  const txId = await connection.sendRawTransaction(signedTx.serialize());

  await connection.confirmTransaction({
    signature: txId,
    blockhash,
    lastValidBlockHeight,
  });

  return txId;
}

/**
 * Sync Komon reputation to SOVEREIGN civic score
 * If user is not the civic authority, sets themselves as authority first
 */
export async function syncToSovereign(
  connection: Connection,
  wallet: WalletContextState,
  reputation: KomonReputation
): Promise<{ txId: string; newScore: number; needsSetup: boolean }> {
  if (!wallet.publicKey || !wallet.signTransaction) {
    throw new Error('Wallet not connected');
  }

  // Fetch current identity
  const identity = await fetchSovereignIdentity(connection, wallet.publicKey);
  if (!identity) {
    throw new Error('No SOVEREIGN identity found. Create one first.');
  }

  // Calculate the new civic score
  const newScore = calculateCivicScore(reputation);

  // Check if user is the civic authority
  const isAuthority = identity.civicAuthority.equals(wallet.publicKey);
  let needsSetup = false;

  if (!isAuthority) {
    // Check if no authority is set (default pubkey)
    const defaultPubkey = new PublicKey('11111111111111111111111111111111');
    if (!identity.civicAuthority.equals(defaultPubkey)) {
      throw new Error('Another authority is already set for civic score. You cannot sync.');
    }

    // Set user as their own civic authority
    needsSetup = true;
    await setCivicAuthority(connection, wallet, wallet.publicKey);

    // Wait for the authority to be set
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }

  // Update the civic score
  const txId = await updateCivicScore(connection, wallet, wallet.publicKey, newScore);

  return { txId, newScore, needsSetup };
}
