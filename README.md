# Komon

**Prediction markets have proven they're the best mechanism humanity has for aggregating information.** They outperform polls, experts, and committees. They reward correct judgment and punish wishful thinking.

But we've wasted them on entertainment—elections, sports, celebrity gossip.

**Komon points that same mechanism at the problems people actually live with.**

---

## The Transformation

Traditional prediction market:
> "Will the city fix potholes this year?"

Komon:
> "Which approach will fix THIS pothole?"

The first generates information. The second generates **actionable intelligence**—and rewards the people who provide it.

This is prediction markets graduating from entertainment to infrastructure.

---

## What Changes

**The civic feedback loop inverts.** Instead of citizens complaining and officials deciding, citizens propose AND evaluate. The signal isn't "we're unhappy"—it's "here's what we believe will work, backed by money."

**A new class of civic participant emerges.** Not activists, not bureaucrats—*direction traders*. People who develop expertise in what actually gets problems solved in their neighborhood.

**Local knowledge becomes valuable.** The person who knows the city council member's priorities, or which contractor does good work, or that the park cleanup always happens in spring—that knowledge becomes monetizable.

**Incentives align toward outcomes.** Everyone in the system—problem posters, direction proposers, stakers, verifiers—only wins when problems actually get solved.

---

## How It Works

1. **See a problem** → Post it with location, description, success criteria
2. **Have an idea?** → Propose a direction (solution approach)
3. **Think something will work?** → Stake money on YES or NO
4. **Be right** → Get paid when the problem is resolved

Your track record follows you. Good judgment compounds. Bad judgment is visible.

No credentials required. No connections needed. Just be right more than you're wrong.

---

## The Name

**Komon** (コモン) comes from "commons"—shared resources that belong to everyone.

Civic problems are commons problems: they affect us all, but no one person owns them.

Shared problems deserve shared solutions.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND                                 │
│  Next.js 15 + React 19 + TypeScript + Tailwind                  │
│  Solana Wallet Adapter (Phantom, Solflare)                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         API LAYER                                │
│  Next.js API Routes + Prisma ORM + Claude AI                    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      SOLANA PROGRAMS                             │
│  - ProblemRegistry: Post problems with evidence                  │
│  - DirectionMarket: Binary outcome markets per solution          │
│  - Treasury: Pool for rewards + fee distribution                 │
│  - Reputation: Track record + SOVEREIGN integration              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SOVEREIGN PROTOCOL                            │
│  Universal identity with multi-dimensional reputation            │
│  Komon syncs civic scores ──► SOVEREIGN identity                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DATA STORAGE                                │
│  PostgreSQL + Solana (immutable records)                        │
└─────────────────────────────────────────────────────────────────┘
```

Web3 is invisible to users—they see USD while the backend handles USDC on Solana.

---

## Deployments

| Network | Program | ID |
|---------|---------|-----|
| Devnet | Reputation | `AWDeGkLSX3HcU2s8vhYThxLeDQ4N9DqhnREuZU36vuJU` |
| Devnet | SOVEREIGN | `2UAZc1jj4QTSkgrC8U9d4a7EM9AQunxMvW5g7rX7Af9T` |

---

## SOVEREIGN Integration

Komon integrates with [SOVEREIGN](https://github.com/psyto/sovereign), a universal identity and multi-dimensional reputation protocol. Your civic participation in Komon contributes to your portable SOVEREIGN identity.

### How It Works

```
Komon Reputation                    SOVEREIGN Identity
─────────────────                   ──────────────────
Win rate          ───┐
Directions won       ├──► Civic Score ──► Composite Score ──► Tier
Level/XP             │
Streak           ───┘
```

### Benefits

- **Portable Reputation**: Your Komon civic score syncs to SOVEREIGN, usable across other apps
- **Tier-Based Access**: Higher SOVEREIGN tiers unlock higher stake limits
- **Cross-App Recognition**: Build reputation in Komon, get recognized in Umbra, Dverse, and more

### Stake Limits by Tier

| Tier | Name | Max Stake |
|------|------|-----------|
| 1 | Bronze | 100 USDC |
| 2 | Silver | 500 USDC |
| 3 | Gold | 2,000 USDC |
| 4 | Platinum | 10,000 USDC |
| 5 | Diamond | Unlimited |

### Syncing Your Reputation

Users can sync their Komon reputation to SOVEREIGN directly from the profile page:

1. Navigate to your profile page
2. View your calculated civic score preview
3. Click "Sync to SOVEREIGN" to update your on-chain identity

The sync calculates your civic score using a weighted formula:

| Metric | Weight | Description |
|--------|--------|-------------|
| Win Rate | 40% | Your prediction accuracy on directions |
| Directions Won | 25% | Tier based on total successful predictions |
| Level/Trust | 25% | Your Komon level as a trust proxy |
| Current Streak | 10% | Bonus for consecutive wins |

```typescript
import { syncToSovereign, calculateCivicScore } from '@/lib/solana/sovereign';

// Calculate what your score would be
const previewScore = calculateCivicScore({
  winRate: 77.8,
  directionsWon: 28,
  directionsProposed: 45,
  currentStreak: 5,
  level: 15,
});

// Sync to SOVEREIGN (sets authority if needed, then updates score)
const result = await syncToSovereign(connection, wallet, reputation);
// result: { txId: "...", newScore: 6850, needsSetup: false }
```

---

## Wallet Connection

Komon integrates Solana wallet adapters for seamless Web3 experience:

- **Supported Wallets**: Phantom, Solflare
- **Network**: Devnet (mainnet coming soon)
- **Features**:
  - Connect wallet from navigation bar
  - View SOVEREIGN identity and tier badge
  - Tier-based stake limits enforced on-chain
  - Real-time SOVEREIGN score display on profile
  - Sync Komon reputation to SOVEREIGN civic score

### Frontend Components

```typescript
// SOVEREIGN tier badge in navigation
<SovereignNavBadge identity={sovereignIdentity} loading={loading} />

// Full tier display with score breakdown
<SovereignTierBadge identity={sovereignIdentity} showDetails size="lg" />

// Stake dialog with tier-based limits
<StakeDialog
  open={open}
  onOpenChange={setOpen}
  directionId={id}
  outcome="YES"
/>

// Sync to SOVEREIGN button (in profile page)
<button onClick={handleSyncToSovereign}>
  Sync to SOVEREIGN
</button>
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [PHILOSOPHY.md](docs/PHILOSOPHY.md) | Why we're building this |
| [MECHANISM.md](docs/MECHANISM.md) | How the system works |
| [ECONOMICS.md](docs/ECONOMICS.md) | Revenue model and sustainability |
| [GOVERNANCE.md](docs/GOVERNANCE.md) | Decision-making and decentralization |
| [VERIFICATION.md](docs/VERIFICATION.md) | How outcomes are verified |
| [USER_JOURNEYS.md](docs/USER_JOURNEYS.md) | Flows for different user types |
| [ROADMAP.md](docs/ROADMAP.md) | Development timeline |
| [FAQ.md](docs/FAQ.md) | Common questions |
| [GLOSSARY.md](docs/GLOSSARY.md) | Term definitions |
| [MANIFESTO.md](docs/MANIFESTO.md) | The short version |

---

## Project Structure

```
komon/
├── programs/                    # Anchor/Solana programs
│   ├── problem-registry/        # Problem management
│   ├── direction-market/        # Prediction markets
│   ├── treasury/               # Fund management
│   └── reputation/             # Soulbound reputation
│       └── src/
│           ├── lib.rs          # Main program
│           └── sovereign/      # SOVEREIGN integration
├── app/                        # Next.js frontend
│   ├── src/
│   │   ├── app/               # Pages and API routes
│   │   ├── components/
│   │   │   ├── ui/            # UI components (navigation)
│   │   │   ├── providers/     # Wallet & app providers
│   │   │   ├── sovereign/     # SOVEREIGN tier components
│   │   │   ├── problems/      # Problem-related components
│   │   │   ├── directions/    # Direction components
│   │   │   └── staking/       # Staking dialog & components
│   │   └── lib/
│   │       ├── utils/         # Utility functions
│   │       └── solana/        # Solana/SOVEREIGN utilities
│   └── prisma/               # Database schema
├── docs/                       # Documentation
├── tests/                     # Anchor tests
└── Anchor.toml
```

---

## Getting Started

### Prerequisites

- Node.js 20+
- Rust and Cargo
- Solana CLI
- Anchor CLI
- PostgreSQL

### Installation

```bash
# Clone and install dependencies
cd komon
npm install
cd app && npm install

# Set up environment
cp .env.example .env
# Edit .env with your configuration

# Initialize database
npx prisma generate
npx prisma db push

# Build Solana programs
cd ..
anchor build

# Run development server
cd app
npm run dev
```

---

## Tech Stack

- **Frontend:** Next.js 15, React 19, TypeScript, Tailwind CSS
- **Backend:** Next.js API Routes, Prisma ORM
- **Database:** PostgreSQL
- **Blockchain:** Solana, Anchor Framework
- **AI:** Claude API (Anthropic)

---

## License

MIT

---

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

---

**Direct the future. Get rewarded.**

*This is Komon.*
