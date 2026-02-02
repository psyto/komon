# Komon FAQ

## General

### What is Komon?

Komon is a protocol for civic problem-solving that combines prediction markets with local community issues. Anyone can post a problem, propose solutions ("directions"), and stake money on which direction will actually work. When problems get solved, people who correctly predicted the winning approach get rewarded.

### What does "Komon" mean?

Komon (コモン) comes from "commons"—shared resources that belong to everyone. Civic problems are commons problems: they affect us all, but no one person owns them. The name reflects our belief that shared problems deserve shared solutions.

### How is this different from 311 apps?

311 apps collect complaints. Komon collects predictions about what will actually work. With 311, you report a problem and hope the city does something. With Komon, you identify the problem AND propose solutions AND stake money on what you believe will work. The city (or anyone) can see not just what's broken, but what the community thinks will fix it.

### How is this different from regular prediction markets?

Traditional prediction markets ask "Will X happen?" Komon asks "Will this solution work?" It's prescription, not just prediction. We're not betting on what the city will do—we're collectively deciding what should be done and backing our judgment with money.

### Is this gambling?

No. Komon is a prediction market focused on civic outcomes—verifiable real-world events like "Was the pothole fixed?" or "Did the park cleanup happen?" Users stake on their beliefs about which solutions will work, similar to how investors allocate capital based on predictions about company success. We focus on civic improvement, not entertainment or speculation.

---

## Using Komon

### Do I need cryptocurrency to use Komon?

No. You see dollars, we handle the crypto. When you stake $10, you're staking $10. Behind the scenes, we convert to USDC on Solana, but you never need to understand wallets, gas fees, or blockchain concepts. It's like Venmo—you don't think about ACH transfers.

### Do I need to create an account?

You can browse problems and directions without an account. To stake, propose directions, or post problems, you'll need to sign up with your email. We create a wallet for you automatically—you'll never see it.

### What's the minimum stake?

$1. We want participation to be accessible, not just for people with money to burn.

### What's the maximum stake?

$1,000 per position (initially). This prevents whales from dominating markets and manages risk during early stages. Limits may change through governance as the protocol matures.

### Can I change my mind after staking?

Yes. You can unstake (sell your position) at any time before resolution. There's a small 1% fee to discourage frivolous position changes, but you're never locked in.

### What happens if no one solves the problem?

If the deadline passes and no direction succeeded, everyone who staked NO on all directions wins. They correctly predicted that nothing would work. The bounty returns to the problem creator (minus fees).

---

## Problems

### What kind of problems can I post?

Civic problems that affect your community: potholes, broken streetlights, illegal dumping, unsafe crosswalks, park maintenance, noise issues, etc. Problems should be:
- **Specific:** "Pothole at Main & Oak" not "Roads are bad"
- **Local:** Tied to a location others can verify
- **Solvable:** Has a realistic path to resolution
- **Verifiable:** Clear criteria for what "solved" means

### What shouldn't I post?

- Personal disputes ("My neighbor is annoying")
- Illegal activity reports (call the police)
- Subjective complaints ("The park isn't pretty enough")
- Problems outside your area (you can't verify outcomes)
- Duplicate problems (search first)

### How do I define success criteria?

Be specific. Instead of "pothole fixed," write:
- Pothole filled with permanent material (not cold patch)
- Surface level with surrounding road (±1 inch)
- Repair persists for at least 7 days

The clearer your criteria, the smoother verification will be.

### Can I add a bounty?

Yes. Bounties attract more attention and increase rewards for successful directions. A 5% fee is taken from bounties. You can add to the bounty anytime before resolution.

### Can I edit my problem after posting?

You can edit the description for clarity, but not the success criteria (that would be unfair to people who already staked). If you made a mistake in the criteria, you may need to close the problem and create a new one.

---

## Directions

### What's a "direction"?

A direction is a proposed solution to a problem. It's not just an idea—it's a hypothesis that the community can stake on. "Call the city" is a direction. "Organize a community cleanup" is a direction. "Wait for the city to notice" is also a direction.

We call them "directions" not "solutions" because:
1. They're paths forward, not guaranteed fixes
2. Multiple directions can be partially right
3. It emphasizes the journey, not just the destination

### How do I propose a good direction?

Be specific and actionable:
- **Weak:** "Someone should do something"
- **Strong:** "File a request with the city's Public Works department using form PW-123, including photos and the specific location. Follow up weekly until addressed."

Good directions include:
- Clear steps anyone could follow
- Realistic assessment of effort/cost
- Why you believe this will work

### Does AI analyze my direction?

Yes. When you propose a direction, Claude (our AI) provides a feasibility analysis: estimated likelihood of success, potential obstacles, cost/time estimates, and comparison to other directions. This helps stakers make informed decisions.

### Can multiple directions win?

Yes. If a problem is solved through a combination of approaches, verifiers can credit up to 3 directions. Stakers on any credited direction share the rewards proportionally.

### What if my direction doesn't get picked?

That's fine—most directions don't win. If you staked YES on your own direction and it didn't work, you lose that stake. But your reputation still benefits from proposing thoughtful directions, even if they don't win.

---

## Staking

### How does staking work?

When you stake on a direction, you're buying outcome tokens:
- **YES tokens:** You believe this direction will help solve the problem
- **NO tokens:** You believe this direction won't work

If the direction is credited when the problem is resolved, YES holders win. If not, NO holders win.

### How are payouts calculated?

Payouts are proportional to your stake and the odds when you staked. If you staked $10 on YES when odds were 30%, and the direction wins, you receive more than if you staked at 70% odds. Early correct predictions are rewarded more.

### What's the fee?

- **Staking:** 0% (no fee to enter)
- **Winning payout:** 2.5%
- **Early unstake:** 1%

You only pay when you win or exit early.

### Can I see what others have staked?

Yes. Total stakes and current odds are visible for all directions. Individual stakers are anonymous unless they choose to be public.

### Is there a strategy?

The best strategy is to have genuine local knowledge. If you know the city council is already planning to fix something, stake YES on "city will handle it." If you know a particular approach worked in a neighboring area, stake YES on that direction. Information advantage matters more than timing tricks.

---

## Verification

### Who decides if a problem is solved?

Verification is multi-stage:

1. **Claim:** Anyone can claim the problem is solved and submit evidence
2. **Initial review:** Verification committee (5 randomly selected high-reputation users) reviews evidence against success criteria
3. **Challenge period:** 24 hours for stakeholders to dispute
4. **Appeal:** If challenged, expanded committee reviews

See [VERIFICATION.md](./VERIFICATION.md) for full details.

### What evidence do I need?

Depends on the problem type:
- **Physical infrastructure:** Before/after photos with location metadata
- **Safety issues:** Official reports or documented changes
- **Environmental:** Photos plus any official data
- **Policy/process:** Documentation of change

### Can verification be wrong?

Yes, which is why we have appeals. If you believe a verification is incorrect:
1. File a challenge with $10 stake
2. Expanded committee reviews
3. If upheld, you get your stake back and original verifiers lose reputation

### How do I become a verifier?

Reach Reputation Level 5+ and maintain high accuracy on previous verifications. Verifiers must not have stakes in the problems they verify.

---

## Reputation

### How does reputation work?

Your reputation reflects your track record:
- **XP:** Points earned from successful predictions, accepted directions, accurate verifications
- **Level:** Your tier (1-20), determining privileges
- **Win Rate:** Percentage of correct predictions
- **Streak:** Consecutive wins/losses

### Is reputation transferable?

No. Reputation is soulbound—tied to your identity and cannot be bought, sold, or transferred. This prevents gaming and ensures reputation reflects real contribution.

### What happens if I lose a lot?

Your reputation and win rate decrease, but there's no "death spiral." You can always rebuild through correct predictions. New users start at Level 1 with a clean slate.

### Do I need reputation to participate?

No. Anyone can stake on directions from day one. Higher reputation unlocks privileges like proposing directions, becoming a verifier, and participating in governance.

---

## Economics

### How does Komon make money?

- 2.5% fee on winning payouts
- 5% fee on bounties
- 1% fee on early unstakes
- Future: Premium features, API access, data products

### Who pays me when I win?

The pool. When you stake, your money joins the pool for that direction. When the outcome is decided, the pool is distributed to winners. Komon doesn't pay you—other stakers do.

### Is my money safe?

Funds are held in audited smart contracts on Solana. We never have custody of your money. The main risks are:
- Smart contract bugs (mitigated by audits and testing)
- Incorrect verification (mitigated by appeals process)
- Your own bad predictions (that's the point)

### What about taxes?

Winnings may be taxable income depending on your jurisdiction. Komon provides transaction history for your records. Consult a tax professional for your specific situation.

---

## Governance

### Who controls Komon?

Currently: The core team, with full transparency.
Future: Progressive decentralization toward community governance.

We start centralized for speed and iteration, then earn trust before distributing control. See [GOVERNANCE.md](./GOVERNANCE.md) for the path to decentralization.

### Can I influence protocol changes?

Yes. Even now:
- Feedback via Discord and GitHub discussions
- Public roadmap with priority voting
- Bug bounty program

As the protocol matures, formal governance mechanisms (voting on parameters, electing council members) will launch.

### What if I disagree with a decision?

Share your perspective in community channels. If trust is fundamentally broken, the code is open source—the community can fork. This is the ultimate check on governance.

---

## Technical

### Why Solana?

- Fast (400ms finality)
- Cheap ($0.00025/transaction)
- Programmable (complex smart contracts)
- Established ecosystem

We need a chain where staking $1 is economically viable. Ethereum L1 fees would make small stakes impossible.

### Is the code open source?

Smart contracts are fully open source and verifiable on-chain. Frontend code will be open sourced after initial security review.

### What about privacy?

Your stakes are on-chain (public) but tied to an anonymous wallet address. Your email and identity are stored off-chain and never shared. You can participate pseudonymously.

### What if Solana goes down?

If Solana experiences downtime, you can't stake or unstake during that period. Your funds remain safe in the smart contract and will be accessible when the network resumes. Problem deadlines may be extended for significant outages.

---

## Getting Started

### How do I start?

1. Browse problems in your neighborhood
2. Read the directions people have proposed
3. Stake on what you believe will work
4. Watch the outcome
5. Collect rewards if you're right

No need to post problems or propose directions initially. Just observing and staking teaches you how the system works.

### What if there are no problems in my area?

Post one! Start with something obvious and easily verifiable—a pothole, a broken streetlight, litter in a park. Seed the market for your neighborhood.

### How can I help Komon grow?

- Post real problems from your neighborhood
- Propose thoughtful directions
- Stake on outcomes you have knowledge about
- Become a verifier when eligible
- Tell civic-minded friends
- Give us feedback

---

## Still Have Questions?

- **Discord:** [link]
- **GitHub Discussions:** [link]
- **Email:** hello@komon.xyz

We read everything. If your question isn't answered here, it might become a new FAQ entry.
