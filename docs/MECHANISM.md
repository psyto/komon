# The Komon Mechanism

## What Makes This Different

Komon isn't a prediction market. It isn't a civic reporting app. It isn't a bounty platform. It's a new primitive that combines elements of all three into something that doesn't exist yet.

### Prediction Markets Predict. Komon Prescribes.

Traditional prediction markets ask: **"Will X happen?"**
- Will the infrastructure bill pass?
- Will the mayor win reelection?
- Will the crime rate decrease?

These are useful for forecasting but don't help *cause* outcomes. Knowing that crime will probably increase doesn't tell you what to do about it.

Komon asks: **"Which approach to solving X will work?"**
- Will a community petition fix the pothole, or should we crowdfund repairs?
- Will installing cameras reduce dumping, or is cleanup and signage more effective?
- Will a crosswalk make this intersection safe, or do we need a full traffic light?

This is prescription, not prediction. You're not betting on what will happen—you're betting on what *should* happen. The market aggregates judgment about solutions, not just probabilities about events.

### Bounties Reward Completion. Komon Rewards Judgment.

Traditional bounty systems pay whoever completes a task:
- Fix this bug: $500
- Find this vulnerability: $1000
- Implement this feature: $2000

Komon pays whoever *correctly identifies* what will work:
- The person who proposed the winning direction shares the pool
- The people who staked YES on that direction share the pool
- The reward goes to judgment, not just execution

This means you don't need to be the person who fixes the pothole. You need to be the person who correctly judged *how* it would get fixed. The retired engineer who proposed contacting a specific city department gets rewarded, even if the actual repair was done by city workers.

### Reporting Apps Collect Complaints. Komon Aggregates Solutions.

311 and FixMyStreet collect problem reports:
- "There's a pothole at 5th and Main"
- "Streetlight out on Oak Street"
- "Graffiti on the bridge"

Komon collects proposed solutions with skin-in-the-game signals:
- "Community petition" — $2,000 staked, 65% YES
- "Crowdfund private repair" — $800 staked, 35% YES
- "Contact city councilmember directly" — $1,500 staked, 72% YES

The difference is signal quality. A complaint tells you something is wrong. A staked prediction tells you what the community believes will fix it, weighted by confidence.

---

## The Core Loop

```
┌─────────────────────────────────────────────────────────────────┐
│  1. PROBLEM                                                      │
│     Someone posts a civic problem with location and deadline     │
│     Optional: Add bounty to incentivize solutions                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. DIRECTIONS                                                   │
│     Anyone can propose a solution (direction)                    │
│     AI provides feasibility analysis                             │
│     Multiple directions compete for the same problem             │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. STAKING                                                      │
│     People stake on directions they believe will work (YES)      │
│     Or stake against directions they think will fail (NO)        │
│     Stakes create prediction markets per direction               │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. RESOLUTION                                                   │
│     Problem gets resolved (or deadline passes)                   │
│     Winning direction is verified                                │
│     Outcome is recorded immutably                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. REWARDS                                                      │
│     YES stakers on winning direction split the pool              │
│     NO stakers on losing directions were right too               │
│     Reputation is updated (wins, losses, streaks)                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Key Design Decisions

### Why Binary Markets (YES/NO)?

Each direction has its own market: "Will this direction work?"

We considered:
- **Single market with multiple outcomes** — Complex, requires different AMM, harder to understand
- **Ranking/voting** — No skin in game, vulnerable to manipulation
- **Binary per direction** — Simple, composable, each direction succeeds or fails independently

Binary markets are also well-understood. The math is simpler. Users understand YES/NO. And it allows for the possibility that *multiple* directions contributed to solving a problem.

### Why Separate Markets Per Direction?

If Direction A has $1000 staked YES and Direction B has $500 staked YES, it doesn't mean A is twice as likely to work. It might mean A has more visibility, or early momentum, or a whale supporter.

Separate markets mean each direction's odds are independent. You can be bullish on A and bearish on B. You can think both will work, or neither. The markets don't force artificial competition.

### Why Soulbound Reputation?

Your track record follows you permanently. You can't:
- Create a new account to escape a bad record
- Sell your reputation to someone else
- Buy credibility without earning it

This creates long-term incentives. A good reputation takes time to build and is valuable. People with strong track records become trusted voices whose directions carry implicit weight.

### Why AI Analysis?

Every direction gets automated feasibility analysis:
- Feasibility score (0-100)
- Strengths and weaknesses
- Cost and time estimates
- Risk assessment

This serves several purposes:
1. **Levels the playing field** — A first-time user's direction gets the same analysis as an experienced user's
2. **Prompts deeper thinking** — Seeing "Risk: Legal complications with working on public roads" makes proposers think twice
3. **Creates common vocabulary** — Everyone sees the same framework for evaluating directions

The AI doesn't decide. It informs. The market still aggregates human judgment—but better-informed judgment.

### Why Web3?

You might notice there's no mention of tokens, wallets, or blockchain in the user experience. That's intentional. Users see dollars. They don't know or care that it's USDC on Solana.

But the blockchain matters:
- **Immutable records** — Problem resolutions and outcomes can't be changed retroactively
- **Permissionless** — No one can be banned from participating
- **Transparent** — All stakes and outcomes are publicly verifiable
- **Programmable** — Smart contracts enforce rules without trusted intermediaries

Web3 is infrastructure, not identity. It enables the mechanism without being the point.

---

## Economic Dynamics

### For Proposers

Proposing a direction is free, but your reputation is at stake. If your direction wins, you get:
- Share of the bounty (if any)
- Reputation boost (XP, win rate, streak)
- Status as the person who solved the problem

If your direction loses, your win rate drops. Propose too many bad directions and your track record speaks for itself.

### For Stakers

Staking is a prediction: "I believe this direction will work."

- **Stake YES** if you think the direction will solve the problem
- **Stake NO** if you think it will fail

If you're right, you split the pool with other correct stakers. If you're wrong, you lose your stake.

This creates pressure to:
1. Research before staking
2. Look for overlooked directions with good odds
3. Update as new information arrives (you can unstake before resolution)

### For Problem Posters

Posting a problem is free. Adding bounty attracts more attention and higher-quality directions.

The poster doesn't pick the winner—the market does, and reality verifies. This prevents favoritism and ensures outcomes matter more than politics.

### Market Making

Initially, directions have no liquidity. Early stakers set the odds. As more stakes come in, the market finds equilibrium.

This creates opportunities for sophisticated participants:
- Identify mispriced directions (good solutions with low YES stakes)
- Provide liquidity by staking on unpopular but viable directions
- Arbitrage between similar problems in different locations

Over time, we expect "civic analysts" to emerge—people who specialize in evaluating civic solutions and have track records to prove it.

---

## What Success Looks Like

### Short Term
- Problems get solved faster because better solutions surface
- People with local knowledge have a venue to contribute
- Cities get signal about what approaches have community support

### Medium Term
- Reputation becomes meaningful—high win-rate users are consulted
- Patterns emerge: "Community petitions work for X, direct action works for Y"
- Data informs policy: which approaches work in which contexts?

### Long Term
- Komon becomes infrastructure for collective decision-making
- Governments, NGOs, and communities use it to prioritize and evaluate
- A new form of civic participation emerges—judgment as contribution

---

## Open Questions

**How do we verify outcomes?**
MVP uses authority-based verification (trusted party confirms resolution). Future versions could use threshold verification (M-of-N verifiers must agree) or even prediction markets on verification itself.

**What prevents manipulation?**
Large stakes from single actors are visible. Sybil attacks (many fake accounts) are limited by reputation requirements. Long-term, identity solutions and stake-weighted verification help.

**What if no direction works?**
If a problem expires without resolution, NO stakers on all directions win. This is correct—they predicted nothing would work, and they were right.

**How do we handle partial solutions?**
A direction might partially work. Initially, we use binary resolution. Future versions could support partial payouts based on degree of success.

---

## The Bet We're Making

Komon bets that:

1. **People have better judgment than they're credited with**—given proper incentives and information
2. **Skin in the game improves decision quality**—across the board
3. **Local problems are tractable**—they're not solved because of coordination failure, not fundamental difficulty
4. **Reputation compounds**—good judges will be recognized and trusted over time

If we're right, Komon becomes essential infrastructure for how communities solve problems together.

If we're wrong, we'll learn something important about the limits of market-based coordination.

Either way, we're building toward a world where good judgment matters more than loud voices, and where shared problems get shared solutions.
