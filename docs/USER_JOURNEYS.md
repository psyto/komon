# Komon User Journeys

## Overview

Four primary user types interact with Komon:

1. **Problem Posters** — Identify and document civic issues
2. **Direction Proposers** — Suggest solutions
3. **Stakers** — Back directions with money
4. **Verifiers** — Confirm outcomes

Most users will play multiple roles over time. A problem poster might also stake on solutions. A prolific staker might become a verifier.

---

## Journey 1: Problem Poster

### Persona: Maria

Maria is a small business owner. A pothole outside her shop has been damaging customers' cars for months. She's called 311 twice with no response.

### Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  DISCOVER                                                        │
│  Maria hears about Komon from a neighbor who got a streetlight  │
│  fixed through the platform.                                     │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ONBOARD                                                         │
│  - Visits komon.io                                               │
│  - Browses existing problems (no account needed)                 │
│  - Signs up with email (wallet created invisibly)                │
│  - Takes 2-minute tour of how it works                           │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  POST PROBLEM                                                    │
│  - Clicks "Post Problem"                                         │
│  - Takes photo of pothole with phone                             │
│  - Drops pin on map for location                                 │
│  - Writes title: "Large pothole on Main St damaging vehicles"    │
│  - Writes description with details                               │
│  - Selects category: Infrastructure                              │
│  - Sets deadline: 30 days                                        │
│  - Defines success criteria:                                     │
│    ✓ Pothole filled with permanent material                      │
│    ✓ Surface level with road                                     │
│    ✓ Repair lasts at least 7 days                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  AI ANALYSIS (Automatic)                                         │
│  - AI reviews problem                                            │
│  - Suggests: "Similar problems resolved by contacting city       │
│    council member directly. Average resolution: 3 weeks."        │
│  - Maria sees this before posting                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ADD BOUNTY (Optional)                                           │
│  - Maria adds $50 bounty to attract attention                    │
│  - Pays with credit card ($50 + $2.50 fee)                       │
│  - Bounty displayed on problem                                   │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  MONITOR                                                         │
│  - Problem goes live                                             │
│  - Maria gets notifications when:                                │
│    • New direction proposed                                      │
│    • Significant staking activity                                │
│    • Someone comments                                            │
│  - She can see which directions have most support                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  RESOLUTION                                                      │
│  - 2 weeks later, pothole is fixed                               │
│  - Maria submits resolution claim with after photo               │
│  - Verification confirms                                         │
│  - Winning direction identified                                  │
│  - Maria's reputation increases (+10 XP for resolved problem)    │
└─────────────────────────────────────────────────────────────────┘
```

### Key Moments

| Moment | Maria's Feeling | Design Goal |
|--------|-----------------|-------------|
| First visit | Curious but skeptical | Show real success stories |
| Sign up | "Is this going to be complicated?" | 30-second signup, no crypto jargon |
| Post problem | Empowered | Make posting feel like taking action |
| See directions | Hopeful | Show community engagement |
| Resolution | Satisfied | Celebrate the win, show impact |

### Success Metrics

- Time to first problem post: <5 minutes
- Problem completion rate: >70%
- Return rate: Posts second problem within 90 days

---

## Journey 2: Direction Proposer

### Persona: James

James is a retired city engineer. He knows how municipal processes work. He sees problems on Komon and often knows exactly what would solve them.

### Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  DISCOVER                                                        │
│  James browses problems in his neighborhood                      │
│  Sees Maria's pothole problem                                    │
│  Thinks: "I know exactly how to get this fixed"                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ANALYZE                                                         │
│  - Reads problem description and success criteria                │
│  - Checks existing directions (2 already proposed)               │
│  - Direction A: "Report to 311 again" — 30% YES                  │
│  - Direction B: "Start online petition" — 45% YES                │
│  - James thinks both are weak                                    │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  PROPOSE DIRECTION                                               │
│  - Clicks "Propose Direction"                                    │
│  - Writes: "Contact city council member directly with photo      │
│    evidence and request expedited repair through Emergency       │
│    Road Maintenance fund. Copy the Public Works director."       │
│  - Adds specifics: "Council member Johnson handles this          │
│    district. Email: johnson@city.gov"                            │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  AI ANALYSIS (Automatic)                                         │
│  - AI evaluates James's direction:                               │
│    • Feasibility: 78%                                            │
│    • Strengths: "Direct escalation path, specific contacts"      │
│    • Weaknesses: "Depends on council member responsiveness"      │
│    • Est. time: 2-3 weeks                                        │
│    • Est. cost: $0 (just effort)                                 │
│  - James reviews, adjusts description slightly                   │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  SUBMIT & STAKE                                                  │
│  - James submits direction                                       │
│  - Option to stake on own direction                              │
│  - Stakes $25 on YES (he's confident)                            │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  WATCH & ENGAGE                                                  │
│  - Direction goes live                                           │
│  - Other users start staking                                     │
│  - James's direction rises to 65% YES                            │
│  - He answers questions in comments                              │
│  - "How do I find the council member email?" — He helps          │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  OUTCOME                                                         │
│  - Problem gets solved following his approach                    │
│  - His direction verified as winner                              │
│  - James receives:                                               │
│    • Share of bounty ($50 × share)                               │
│    • Winnings from YES stake                                     │
│    • +50 XP and win streak extended                              │
│  - His profile shows: "15 wins, 3 losses, 83% win rate"          │
└─────────────────────────────────────────────────────────────────┘
```

### Key Moments

| Moment | James's Feeling | Design Goal |
|--------|-----------------|-------------|
| See weak directions | "I can do better" | Show opportunity |
| Write direction | Confident in expertise | Easy to add detail |
| See AI analysis | Validated | AI confirms his instincts |
| Others stake YES | Respected | Community recognizes quality |
| Win | Proud | Reputation reflects expertise |

### Success Metrics

- Directions per active proposer: >3/month
- Proposer win rate: >50% (self-selection working)
- Repeat proposal rate: >60%

---

## Journey 3: Staker

### Persona: Alex

Alex is a data analyst who loves prediction markets. They use Polymarket for elections and sports. They heard about Komon and want to try applying prediction skills to local issues.

### Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  DISCOVER                                                        │
│  Alex sees Komon mentioned in a prediction market forum          │
│  "Like Polymarket but for local problems"                        │
│  Intrigued by the novelty                                        │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  EXPLORE                                                         │
│  - Creates account                                               │
│  - Browses problems sorted by:                                   │
│    • Highest bounty                                              │
│    • Most staked                                                 │
│    • Ending soon                                                 │
│    • Near me                                                     │
│  - Looks for mispriced directions                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ANALYZE                                                         │
│  - Finds Maria's pothole problem                                 │
│  - 3 directions now:                                             │
│    A: "311 report" — 20% YES ($100 staked)                       │
│    B: "Online petition" — 35% YES ($200 staked)                  │
│    C: "Contact council member" — 60% YES ($400 staked)           │
│  - Reads AI analysis for each                                    │
│  - Checks proposers' track records:                              │
│    • A proposer: 2 wins, 5 losses (29%)                          │
│    • B proposer: 3 wins, 3 losses (50%)                          │
│    • C proposer (James): 15 wins, 3 losses (83%)                 │
│  - Alex thinks C is undervalued given James's record             │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  STAKE                                                           │
│  - Decides to stake $50 YES on Direction C                       │
│  - Adds funds via credit card (first time)                       │
│  - Confirms stake                                                │
│  - Also stakes $10 NO on Direction A (thinks 311 won't work)     │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  PORTFOLIO                                                       │
│  - Alex's dashboard shows:                                       │
│    • Active stakes: 2                                            │
│    • Total staked: $60                                           │
│    • Potential return: $85 (if both correct)                     │
│  - Gets notifications on significant market movements            │
│  - Can unstake anytime (1% fee)                                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  OUTCOME                                                         │
│  - Direction C wins (contact council member worked)              │
│  - Direction A loses (311 didn't help)                           │
│  - Alex receives:                                                │
│    • YES stake on C: $50 → $72 (+$22)                            │
│    • NO stake on A: $10 → $15 (+$5)                              │
│    • Total profit: $27                                           │
│    • +10 XP, 2-win streak started                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  REPEAT                                                          │
│  - Alex is hooked                                                │
│  - Starts looking for more mispriced directions                  │
│  - Develops expertise in certain problem categories              │
│  - Win rate: 62% after 20 stakes                                 │
│  - Leaderboard rank: #47                                         │
└─────────────────────────────────────────────────────────────────┘
```

### Key Moments

| Moment | Alex's Feeling | Design Goal |
|--------|-----------------|-------------|
| Browse markets | Analytical, hunting | Good filtering and sorting |
| See proposer records | Informed | Reputation visible and useful |
| Place stake | Committed | Smooth transaction flow |
| Watch market move | Engaged | Real-time updates |
| Win | Validated | Clear P&L display |

### Success Metrics

- Stakes per active staker: >5/month
- Average stake size: $25-50
- Staker retention (90-day): >40%

---

## Journey 4: Verifier

### Persona: Diana

Diana has been active on Komon for 6 months. She's Level 7 with a 71% win rate. She gets invited to become a verifier.

### Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  INVITATION                                                      │
│  - Diana receives notification: "You're eligible to verify!"     │
│  - Requirements met:                                             │
│    ✓ Level 5+                                                    │
│    ✓ 20+ resolved stakes                                         │
│    ✓ No conduct violations                                       │
│  - She opts in to verifier pool                                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  TRAINING                                                        │
│  - Completes short verification training:                        │
│    • What makes good evidence?                                   │
│    • How to evaluate success criteria?                           │
│    • What to do in edge cases?                                   │
│  - Passes quiz (3 example cases)                                 │
│  - Becomes active verifier                                       │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ASSIGNMENT                                                      │
│  - Gets notification: "Verification request"                     │
│  - Randomly assigned to Maria's pothole problem                  │
│  - Has 48 hours to review                                        │
│  - Cannot verify if she has stake in this problem                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  REVIEW                                                          │
│  - Diana sees:                                                   │
│    • Original problem + success criteria                         │
│    • Resolution claim + evidence                                 │
│    • Before/after photos                                         │
│    • Winning direction claimed                                   │
│  - She evaluates:                                                │
│    ✓ Location matches (GPS data confirms)                        │
│    ✓ Repair is visible and appears permanent                     │
│    ✓ Date is after problem posted                                │
│    ? Surface level hard to judge from photo                      │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  DECISION                                                        │
│  - Diana submits vote: VERIFIED                                  │
│  - Adds note: "Surface appears level, GPS confirmed"             │
│  - Vote is encrypted until all 5 verifiers submit                │
│  - She doesn't know others' votes yet                            │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  RESULT                                                          │
│  - All 5 votes in: 4 Verified, 1 Not Verified                    │
│  - 3-of-5 threshold met → VERIFIED                               │
│  - Diana was in majority                                         │
│  - She receives:                                                 │
│    • +10 XP for accurate verification                            │
│    • "Trusted Verifier" progress (8/10 accurate)                 │
│  - Dissenting verifier loses no XP (minority opinion OK)         │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  ONGOING                                                         │
│  - Diana verifies 2-3 problems per week                          │
│  - Her accuracy stays above 80%                                  │
│  - After 20 verifications, earns "Trusted Verifier" badge        │
│  - Gets priority for high-value verifications                    │
│  - Eligible for future verification rewards                      │
└─────────────────────────────────────────────────────────────────┘
```

### Key Moments

| Moment | Diana's Feeling | Design Goal |
|--------|-----------------|-------------|
| Invited | Recognized | Clear why she qualified |
| Training | Prepared | Short but thorough |
| Review evidence | Responsible | Good evidence presentation |
| Submit vote | Independent | No influence from others |
| Accurate result | Validated | Feedback on accuracy |

### Success Metrics

- Verifier acceptance rate: >50% of eligible
- Average verification time: <24 hours
- Verifier accuracy: >85%
- Verifier retention: >70% after 10 verifications

---

## Cross-Journey Interactions

```
                    ┌─────────────┐
                    │   PROBLEM   │
                    │   POSTER    │
                    └──────┬──────┘
                           │ posts
                           ▼
                    ┌─────────────┐
                    │   PROBLEM   │
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
   │ DIRECTION A │  │ DIRECTION B │  │ DIRECTION C │
   │  (Proposer) │  │  (Proposer) │  │  (Proposer) │
   └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
          │                │                │
          └────────────────┼────────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   STAKERS   │
                    │ (Multiple)  │
                    └──────┬──────┘
                           │ stakes
                           ▼
                    ┌─────────────┐
                    │  RESOLUTION │
                    │    CLAIM    │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  VERIFIERS  │
                    │   (5 of N)  │
                    └──────┬──────┘
                           │ verifies
                           ▼
                    ┌─────────────┐
                    │   PAYOUT    │
                    └─────────────┘
```

---

## Onboarding Priorities

### First-Time User (Any Type)

1. **See value immediately** — Show resolved problems with real impact
2. **No crypto friction** — Email signup, credit card payment
3. **Quick win** — First stake takes <2 minutes
4. **Clear next step** — Always obvious what to do next

### Progression Path

```
Visitor → Browser → Staker → Proposer → Verifier → Leader
   │         │         │         │          │         │
   │         │         │         │          │         └─ Top 100
   │         │         │         │          └─ Level 5+, trained
   │         │         │         └─ First direction proposed
   │         │         └─ First stake placed
   │         └─ Account created
   └─ No account, just looking
```

Each transition should feel natural and earned, not forced.
