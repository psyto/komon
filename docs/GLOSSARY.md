# Komon Glossary

A reference guide to terms used in Komon.

---

## Core Concepts

### Problem
A civic issue posted to Komon for the community to solve. Problems have a title, description, location, category, success criteria, deadline, and optional bounty. Examples: pothole, broken streetlight, illegal dumping.

### Direction
A proposed solution to a problem. Called "direction" (not "solution") because it represents a path forward rather than a guaranteed fix. Users stake on whether directions will work. Examples: "Contact city Public Works," "Organize community cleanup," "Wait for natural resolution."

### Stake
Money placed on a direction's outcome. Users stake on YES (direction will work) or NO (direction won't work). Stakes determine payouts when problems are resolved.

### Bounty
Optional reward money added to a problem by its creator. Bounties increase the total payout pool and attract more attention and directions. A 5% fee is charged on bounties.

### Resolution
The moment when a problem is verified as solved (or not). Resolution triggers payouts to winning stakers.

### Verification
The process of confirming whether a problem has been solved according to its success criteria. Involves evidence submission, committee review, and possible appeals.

---

## Staking & Markets

### YES Token
An outcome token representing belief that a direction will work. If the direction is credited in resolution, YES holders win.

### NO Token
An outcome token representing belief that a direction won't work. If the direction is not credited in resolution, NO holders win.

### Outcome Token
General term for YES and NO tokens. These are digital assets that represent your stake on a particular outcome. They're not transferable between users.

### Position
Your total stake on a direction, including all YES and NO tokens you hold. "Opening a position" means staking; "closing a position" means unstaking.

### Odds
The implied probability of an outcome based on current stakes. If YES has $70 and NO has $30, the implied odds are 70% for YES. Odds change as people stake.

### Pool
The total money staked on a direction (YES + NO). When outcome is decided, the pool is distributed to winners after fees.

### Payout
Money received by winners after resolution. Calculated proportionally based on stake size and odds at time of staking. Subject to 2.5% fee.

### Unstake
Selling your position before resolution. Subject to 1% fee. Allows you to exit if you change your mind.

### Liquidity
The amount of money available in a market. Higher liquidity means prices are more stable and large stakes have less impact on odds.

---

## Reputation

### Reputation
A non-transferable score reflecting your track record on Komon. Components include XP, level, win rate, and streaks.

### XP (Experience Points)
Points earned through successful activity: correct predictions, accepted directions, accurate verifications. Used to determine level.

### Level
Your tier in the reputation system (1-20). Higher levels unlock privileges like proposing directions, becoming a verifier, and governance participation.

### Win Rate
Percentage of your predictions that were correct. Displayed as a number (e.g., 62%).

### Streak
Consecutive wins or losses. Positive streaks show momentum; negative streaks indicate rough patches. Long positive streaks earn bonus reputation.

### Soulbound
A property meaning something cannot be transferred to another user. Komon reputation is soulbound—it belongs to you alone and reflects your actual contributions.

---

## Verification Terms

### Success Criteria
Specific, measurable conditions that define what "solved" means for a problem. Set by the problem creator at posting time. Examples: "Pothole filled with permanent material," "Surface level within 1 inch."

### Evidence
Documentation submitted to prove a problem was solved. Can include photos, official reports, witness statements, news coverage.

### Claimant
The person who submits a resolution claim and evidence asserting a problem is solved. Can be anyone—problem creator, direction proposer, or community member.

### Verifier
A high-reputation user who reviews evidence and votes on whether problems are solved. Must not have stakes in problems they verify.

### Verification Committee
A group of 5 randomly selected verifiers who decide initial resolution. Requires 3-of-5 agreement.

### Challenge
A formal dispute of a verification decision. Requires $10 stake (refunded if successful). Triggers expanded review.

### Appeal
An escalated review after a challenge. Uses expanded committee (9 verifiers, 6-of-9 agreement required).

### Governance Council
A 7-member elected body that handles final appeals and precedent-setting decisions.

---

## Users & Roles

### Problem Creator / Poster
User who posts a problem to Komon. Defines success criteria and deadline. Can add bounty.

### Direction Proposer
User who proposes a direction (solution) to a problem. Must have reached appropriate reputation level.

### Staker
User who stakes money on directions. Anyone can stake regardless of reputation level.

### Civic Champion
A power user who actively seeds problems, proposes directions, and helps onboard others in their neighborhood.

### Authority
In early phases, the core team acting as centralized verifier. Will be replaced by committee-based verification.

---

## Technical Terms

### Smart Contract
Code running on Solana that manages problems, stakes, and payouts. Executes automatically based on rules—no human can change outcomes arbitrarily.

### PDA (Program Derived Address)
A deterministic wallet address generated by Solana programs. Used to store problem and direction data on-chain.

### USDC
A stablecoin pegged to the US dollar. Komon uses USDC for all stakes and payouts. Users see dollars; USDC is handled behind the scenes.

### Wallet
A digital account that holds cryptocurrency. Komon creates wallets automatically for users—you never need to manage one directly.

### Wallet Abstraction
The technique of hiding blockchain complexity from users. You stake $10, not "10 USDC to PDA xyz." The frontend handles all crypto operations.

### Transaction
An action recorded on the Solana blockchain: staking, unstaking, resolution, payout. Each transaction has a small fee (~$0.00025).

### Devnet
Solana's test network using fake money. Used for development and testing before mainnet launch.

### Mainnet
Solana's production network using real money. Where Komon will operate after MVP testing.

### RPC (Remote Procedure Call)
The interface for communicating with the Solana blockchain. Komon uses RPC providers like Helius for reliable access.

---

## Creator Mode (Vitalik Model)

Komon implements a dual-mode architecture. In addition to civic problems, it supports creator curation based on [Vitalik Buterin's creator coin model](https://vitalik.eth.limo/general/2025/01/23/creatorcoins.html).

### Creator
A content creator seeking admission to a quality-curating DAO. In the shared core, this is called a "Subject" but framed as "Creator" in creator mode.

### Creator DAO
A non-token-based organization of content curators (max 200 members) who vote on which creators to admit. DAOs are opinionated and specialize by content type.

### Content Type
Category of creative work a DAO focuses on: LongFormWriting, ShortFormWriting, Music, ShortFormVideo, LongFormVideo, Fiction, Educational, Podcasts, Art, Code.

### Scout
A prediction market participant who identifies quality creators before mainstream recognition. Scouts stake on whether DAOs will accept creators.

### Admission Prediction
A prediction market on whether a DAO will accept a specific creator. Called "Market" in the shared core.

### Nomination
A DAO member's proposal to consider a creator for admission. Triggers a voting period.

### DAO Vote
The resolution mechanism for creator mode. Members vote Accept/Reject/Abstain with semi-anonymous salt-hashed votes. Requires quorum and threshold.

### Quorum
Minimum percentage of DAO members who must vote for the result to be valid (e.g., 50%).

### Threshold
Minimum approval rate required for admission (e.g., 66% of decisive votes).

### Burn
In creator mode, 5% of gross payout is burned (transferred to burn treasury). Creates deflationary pressure and long-term alignment.

### DAO Membership
On-chain record proving someone is an active member of a Creator DAO. Required to vote, nominate, or add new members.

### Vote Record
On-chain record preventing double voting. Stores keccak hash of voter+creator+salt for semi-anonymity.

---

## Governance Terms

### Protocol
The complete system of rules, smart contracts, and interfaces that make up Komon. "The protocol" refers to Komon as a whole.

### Parameter
A configurable value in the protocol: fee rates, stake limits, verification thresholds, etc. Parameters can be changed through governance.

### DAO (Decentralized Autonomous Organization)
A governance structure where decisions are made by token or reputation-weighted voting rather than a central authority. Komon has two types: (1) Protocol Governance DAO for protocol decisions, and (2) Creator DAOs for content curation (see Creator Mode section).

### Multisig
A wallet requiring multiple signatures to execute transactions. Used for emergency actions and treasury management. "3-of-5 multisig" means 3 of 5 designated signers must approve.

### Time Lock
A delay between when a governance decision is made and when it takes effect. Gives community time to react to changes.

### Supermajority
A threshold higher than simple majority, typically 67%. Required for major protocol changes.

### Fork
Creating a copy of open-source code to run independently. The ultimate check on governance—if the community loses trust, they can fork the protocol.

---

## Economic Terms

### Volume
Total money flowing through the protocol. Higher volume = more fees = more sustainability.

### Fee Rate
Percentage taken by the protocol from various actions. Payout fee: 2.5%, Bounty fee: 5%, Unstake fee: 1%.

### Unit Economics
The revenue and costs associated with a single problem. Determines whether the protocol can be sustainable.

### Breakeven
The activity level at which protocol revenue equals costs. Komon breaks even at approximately 750 problems/month.

### Treasury
The protocol's reserve of funds, accumulated from fees. Used for operations, grants, and growth initiatives.

### Cold Start
The challenge of getting initial activity when there's no existing user base. Komon addresses this through seeding, bounties, and neighborhood focus.

---

## Categories

### Infrastructure
Problems related to physical structures: roads, bridges, sidewalks, utilities. Examples: potholes, cracked sidewalks, broken pipes.

### Safety
Problems affecting personal safety: traffic, lighting, crime. Examples: dangerous intersections, dark alleys, abandoned buildings.

### Environment
Problems affecting natural surroundings: pollution, parks, wildlife. Examples: illegal dumping, dead trees, contaminated water.

### Noise
Problems related to sound: construction, traffic, establishments. Examples: late-night bar noise, construction outside permitted hours.

### Services
Problems with public services: transit, garbage, mail. Examples: missed trash pickup, broken bus stop, mail theft.

### Other
Problems that don't fit standard categories. Use sparingly—most issues fit existing categories.

---

## Statuses

### Problem Statuses

| Status | Meaning |
|--------|---------|
| Open | Accepting directions and stakes |
| In Progress | Active work happening, still accepting stakes |
| Under Verification | Resolution claimed, being reviewed |
| Resolved | Verified as solved, payouts distributed |
| Closed | Deadline passed without resolution |
| Disputed | Verification challenged, under appeal |

### Direction Statuses

| Status | Meaning |
|--------|---------|
| Proposed | Submitted, accepting stakes |
| Active | Has significant stakes, being considered |
| Credited | Verified as contributing to resolution |
| Not Credited | Problem resolved but this direction wasn't the cause |
| Failed | Problem closed without resolution |

---

## Metrics

### Problems Posted
Total problems created on the platform.

### Problems Resolved
Problems verified as solved.

### Resolution Rate
Percentage of problems that get solved: (Resolved / Closed or Resolved).

### Total Volume
Sum of all stakes placed.

### Active Users
Users who staked, posted, or proposed in the last 30 days.

### Verifier Accuracy
Percentage of verifications not overturned on appeal.

### NPS (Net Promoter Score)
Measure of user satisfaction: "How likely are you to recommend Komon?" Scores range -100 to +100.

---

## Abbreviations

| Abbreviation | Full Term |
|--------------|-----------|
| MVP | Minimum Viable Product |
| UX | User Experience |
| API | Application Programming Interface |
| PDA | Program Derived Address |
| DAO | Decentralized Autonomous Organization |
| XP | Experience Points |
| NPS | Net Promoter Score |
| RPC | Remote Procedure Call |
| USDC | USD Coin (stablecoin) |
| SOL | Solana (native token) |

---

## See Also

- [MECHANISM.md](./MECHANISM.md) — How Komon works in detail
- [ECONOMICS.md](./ECONOMICS.md) — Revenue model and unit economics
- [GOVERNANCE.md](./GOVERNANCE.md) — Decision-making and decentralization
- [VERIFICATION.md](./VERIFICATION.md) — How outcomes are verified
- [FAQ.md](./FAQ.md) — Common questions answered
