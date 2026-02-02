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

Web3 is invisible to users—they see USD while the backend handles USDC on Solana.

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
├── app/                        # Next.js frontend
│   ├── src/
│   │   ├── app/               # Pages and API routes
│   │   ├── components/        # React components
│   │   └── lib/              # Utilities and helpers
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
