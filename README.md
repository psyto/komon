# Komon

**Komon** (コモン) - A global protocol for civic problem-solving. From "commons" - shared problems deserve shared solutions.

> "Direct the future. Get rewarded."

## Overview

Komon is a decentralized platform where anyone can:
- **Identify local problems** - Post civic issues in your community
- **Propose solutions** - Suggest directions with AI-powered feasibility analysis
- **Stake on outcomes** - Bet on which solutions will work
- **Get rewarded** - Earn when your predictions are correct

Web3 is invisible to users - they see USD while the backend handles USDC on Solana.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND                                 │
│  Next.js 15 + React 19 + TypeScript + Tailwind                  │
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
│  - Reputation: Soulbound tokens for track record                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DATA STORAGE                                │
│  PostgreSQL + Solana (immutable records)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
komon/
├── programs/                    # Anchor/Solana programs
│   ├── problem-registry/        # Problem management
│   ├── direction-market/        # Prediction markets
│   ├── treasury/               # Fund management
│   └── reputation/             # Soulbound reputation
├── app/                        # Next.js frontend
│   ├── src/
│   │   ├── app/               # Pages and API routes
│   │   ├── components/        # React components
│   │   └── lib/              # Utilities and helpers
│   └── prisma/               # Database schema
├── tests/                     # Anchor tests
├── Anchor.toml
└── README.md
```

## Getting Started

### Prerequisites

- Node.js 20+
- Rust and Cargo
- Solana CLI
- Anchor CLI
- PostgreSQL

### Installation

1. **Clone and install dependencies:**
   ```bash
   cd komon
   npm install
   cd app && npm install
   ```

2. **Set up environment:**
   ```bash
   cd app
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Initialize database:**
   ```bash
   npx prisma generate
   npx prisma db push
   ```

4. **Build Solana programs:**
   ```bash
   cd ..
   anchor build
   ```

5. **Run development server:**
   ```bash
   cd app
   npm run dev
   ```

### Deploy to Devnet

```bash
# Configure Solana for devnet
solana config set --url devnet

# Deploy programs
anchor deploy

# Update program IDs in Anchor.toml and .env
```

## Smart Contracts

### ProblemRegistry

Manages civic problems with:
- Problem creation with location, category, deadline
- Bounty funding
- Status management (Open → InProgress → Resolved/Closed)

### DirectionMarket

Binary prediction markets for proposed solutions:
- Direction proposals with AI analysis
- YES/NO token minting on stake
- Settlement and reward distribution

### Treasury

Central fund management:
- USDC deposits and allocations
- Fee collection (configurable rate)
- Payout processing

### Reputation

Soulbound (non-transferable) reputation tracking:
- Problems posted, directions proposed
- Win/loss records and streaks
- XP and leveling system

## API Routes

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/problems` | GET, POST | List/create problems |
| `/api/problems/[id]` | GET, PATCH, DELETE | Problem CRUD |
| `/api/directions` | GET, POST | List/create directions |
| `/api/stakes` | GET, POST | Manage stakes |
| `/api/leaderboard` | GET | User rankings |
| `/api/ai/analyze` | POST | AI analysis for problems/directions |

## Tech Stack

- **Frontend:** Next.js 15, React 19, TypeScript, Tailwind CSS
- **Backend:** Next.js API Routes, Prisma ORM
- **Database:** PostgreSQL
- **Blockchain:** Solana (Devnet), Anchor Framework
- **AI:** Claude API (Anthropic)

## Key Features

- **AI-Powered Analysis:** Every problem and direction gets automated feasibility assessment
- **Prediction Markets:** Stake on solutions you believe in
- **Reputation System:** Build credibility through successful predictions
- **Wallet Abstraction:** Users interact with USD, never see crypto

## Development

### Running Tests

```bash
# Anchor tests (requires local validator)
anchor test

# Frontend tests
cd app && npm run test
```

### Building for Production

```bash
cd app
npm run build
```

## License

MIT

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request
