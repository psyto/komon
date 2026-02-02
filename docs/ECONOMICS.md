# Komon Economics

## The Short Version

Komon takes a small fee when money moves through the protocol. More problems solved = more volume = more revenue. Our incentives align with users: we only make money when the system works.

---

## Revenue Streams

### 1. Protocol Fees (Primary)

**Transaction fees on stakes and payouts.**

| Action | Fee | Example |
|--------|-----|---------|
| Stake placed | 0% | User stakes $100 → $100 enters pool |
| Payout to winners | 2.5% | User wins $100 → receives $97.50 |
| Early unstake | 1% | User withdraws $100 before resolution → receives $99 |

**Why this structure:**
- Zero fee to stake = low friction to participate
- Fee on winnings = users only pay when they succeed
- Small unstake fee = discourages frivolous position changes

**At scale:**
- 10,000 problems/year
- Average $500 total staked per problem
- $5M annual volume
- 2.5% average fee = **$125K revenue**

### 2. Bounty Fees

**Fee on bounties posted by problem creators.**

- Problem creator adds $1,000 bounty
- 5% platform fee = $50
- $950 goes to the reward pool

This captures value from the demand side (people who want problems solved), not just the supply side (people solving them).

### 3. Premium Features (Future)

**For power users and institutions:**

| Feature | Price | User |
|---------|-------|------|
| API access | $99/month | Developers, researchers |
| Analytics dashboard | $49/month | Civic analysts, journalists |
| Verified organization badge | $199/year | NGOs, community groups |
| Private problems | $29/problem | Sensitive issues, corporate |
| Priority AI analysis | $9/analysis | Detailed feasibility reports |

### 4. Data and Insights (Future)

**Aggregated, anonymized data products:**

- City governments: "Which solution types work for which problem categories?"
- Urban planners: "What civic issues are emerging in this area?"
- Researchers: "How does stake-weighted prediction compare to expert opinion?"

Priced per contract, likely $10K-$100K for institutional buyers.

### 5. Partnerships (Future)

**Integration with existing civic infrastructure:**

- 311 systems: Problems reported to 311 automatically create Komon entries
- City dashboards: Komon signals feed into municipal decision-making tools
- Insurance companies: Data on neighborhood problem resolution rates

Revenue share or licensing model.

---

## Unit Economics

### Per Problem

```
Average bounty:                    $200
Average total staked:              $500
Total volume per problem:          $700

Revenue:
  Bounty fee (5%):                 $10
  Payout fees (2.5% of stakes):    $12.50
  Total revenue per problem:       $22.50

Costs:
  AI analysis (Claude API):        $0.15
  Solana transaction fees:         $0.02
  Infrastructure (allocated):      $1.00
  Total cost per problem:          $1.17

Gross margin per problem:          $21.33 (95%)
```

### Breakeven Analysis

**Fixed costs (estimated monthly):**
- Infrastructure (hosting, RPC): $500
- Team (2 people, early stage): $15,000
- Legal/compliance: $1,000
- Total: $16,500/month

**Breakeven:**
- $16,500 / $22.50 per problem = **734 problems/month**
- Or ~25 problems/day

### Path to Profitability

| Stage | Problems/month | Monthly Revenue | Status |
|-------|----------------|-----------------|--------|
| Launch | 50 | $1,125 | Loss |
| Traction | 200 | $4,500 | Loss |
| Growth | 750 | $16,875 | Breakeven |
| Scale | 3,000 | $67,500 | Profitable |
| Mature | 10,000 | $225,000 | Highly profitable |

---

## Growth Model

### Phase 1: Seed (Months 1-3)

**Strategy:** Founder-led, single neighborhood

- Pick one neighborhood with active civic engagement
- Seed 10-20 real problems manually
- Recruit 50-100 early users through community groups
- Goal: Prove one problem can be solved through the platform

**Metrics:**
- 1 neighborhood
- 20 problems
- 100 users
- 1 verified resolution

### Phase 2: Validate (Months 4-6)

**Strategy:** Expand to adjacent neighborhoods, find repeatable playbook

- Partner with 2-3 neighborhood associations
- Identify "civic champions" who seed problems
- Test different problem categories (infrastructure vs. safety vs. environment)
- Goal: Demonstrate pattern, not just one-off success

**Metrics:**
- 5 neighborhoods
- 100 problems
- 500 users
- 10 verified resolutions

### Phase 3: Grow (Months 7-12)

**Strategy:** City-wide expansion, press coverage

- Launch in full city
- PR push: "Community solves pothole problem city ignored for years"
- Integrate with local news (problems as story leads)
- Goal: Become known entity in one city

**Metrics:**
- 1 city (all neighborhoods)
- 1,000 problems
- 5,000 users
- 100 verified resolutions

### Phase 4: Scale (Year 2+)

**Strategy:** Multi-city expansion, institutional partnerships

- Replicate playbook in new cities
- Partner with civic tech organizations
- API for 311 integration
- Goal: Network effects across cities

**Metrics:**
- 10 cities
- 10,000 problems
- 50,000 users
- 1,000 verified resolutions

---

## Competitive Landscape

### vs. 311 Apps (SeeClickFix, PublicStuff)

| Dimension | 311 Apps | Komon |
|-----------|----------|-------|
| Input | Problem reports | Problems + solutions |
| Signal | Volume of complaints | Stake-weighted predictions |
| Incentive | None (altruism) | Financial reward |
| Feedback | Maybe a status update | Market prices, resolution, payout |
| Outcome focus | No | Yes |

**Our advantage:** We don't just collect complaints—we aggregate judgment about what will work.

### vs. Prediction Markets (Polymarket, Kalshi)

| Dimension | Prediction Markets | Komon |
|-----------|-------------------|-------|
| Questions | "Will X happen?" | "Will this solution work?" |
| Scope | National/global events | Local civic problems |
| Participation | Speculators | Community members |
| Real-world impact | Informational | Actionable |

**Our advantage:** Prescription, not just prediction. Local, not just national.

### vs. Bounty Platforms (Gitcoin, HackerOne)

| Dimension | Bounty Platforms | Komon |
|-----------|-----------------|-------|
| Task | Defined work | Undefined—propose your approach |
| Reward | Completion | Correct judgment |
| Competition | Winner-take-all | Proportional to stake |

**Our advantage:** You don't need to do the work—just correctly identify what will work.

### vs. Participatory Budgeting

| Dimension | Participatory Budgeting | Komon |
|-----------|------------------------|-------|
| Frequency | Annual | Continuous |
| Scope | Pre-defined options | Open proposals |
| Skin in game | None (voting) | Financial stake |
| Feedback loop | Slow (years) | Fast (weeks/months) |

**Our advantage:** Continuous, stake-weighted, outcome-focused.

---

## Risks and Mitigations

### Regulatory Risk

**Risk:** Prediction markets face regulatory scrutiny. Could Komon be classified as gambling or an unregistered securities platform?

**Mitigation:**
- Focus on civic outcomes, not financial speculation
- Small stakes (cap at $100/position initially)
- Outcomes are verifiable real-world events, not abstract
- Partner with civic institutions for legitimacy
- Legal structure as a "civic engagement platform," not a betting platform

### Adoption Risk

**Risk:** People won't stake real money on civic problems.

**Mitigation:**
- Start with paper trading to build habit
- Low minimum stakes ($1)
- Social proof through leaderboards
- Partner with community groups who have engaged members
- Bounties provide upside even with small stakes

### Manipulation Risk

**Risk:** Bad actors game the system—sybil attacks, wash trading, outcome manipulation.

**Mitigation:**
- Soulbound reputation limits sybil value
- Stake-weighted verification for outcomes
- AI-powered anomaly detection (from fabrknt patterns)
- Progressive trust levels (new users have limits)
- Transparent, auditable on-chain records

### Cold Start Risk

**Risk:** No liquidity without users, no users without liquidity.

**Mitigation:**
- Seed initial problems with bounties
- Protocol-funded liquidity for early directions
- Focus on single neighborhood to concentrate activity
- Gamification (XP, levels) provides non-financial incentive early

### Outcome Verification Risk

**Risk:** Who decides if a direction "worked"? Disputed outcomes could undermine trust.

**Mitigation:**
- Clear success criteria defined upfront
- Multi-party verification (M-of-N verifiers)
- Appeal process with stake-weighted arbitration
- Start with easily verifiable outcomes (pothole fixed = yes/no)

---

## Why Now?

### Crypto infrastructure is ready
- Solana handles thousands of TPS at <$0.01/tx
- Wallet abstraction means users never see crypto
- Stablecoins (USDC) provide stable unit of account

### Civic trust is at historic lows
- Only 20% of Americans trust local government to do what's right
- 311 systems are seen as black holes
- People want agency, not just voice

### Prediction markets are mainstream
- Polymarket did $1B+ volume in 2024
- Cultural acceptance of "betting on outcomes"
- Proven that markets aggregate information well

### Remote work changed neighborhoods
- People are home during the day
- More aware of local problems
- More invested in immediate community

---

## Funding Strategy

### Pre-seed ($250K)

**Use of funds:**
- 2 engineers for 6 months
- Legal setup
- Single-city pilot

**Milestones:**
- Working product
- 100 users
- 10 resolved problems
- Initial regulatory clarity

### Seed ($1.5M)

**Use of funds:**
- Team of 5 for 12 months
- Multi-city expansion
- Partnerships with civic orgs

**Milestones:**
- 10 cities
- 5,000 users
- 500 resolved problems
- Revenue >$50K

### Series A ($8M)

**Use of funds:**
- Team of 15
- Institutional partnerships
- API platform
- Data products

**Milestones:**
- 50 cities
- 100,000 users
- Breakeven or profitable
- Government partnerships

---

## The Big Picture

Komon isn't building an app. We're building **infrastructure for collective decision-making**.

The business model scales with adoption:
- More users → more stakes → more fees
- Better data → better AI → better directions
- More resolutions → more trust → more users

If we're right that stake-weighted prediction produces better civic outcomes, Komon becomes essential infrastructure—a protocol that cities, NGOs, and communities rely on to understand what their constituents believe will actually work.

That's not a $10M company. That's a $1B+ protocol.

---

## Summary

| Question | Answer |
|----------|--------|
| How do we make money? | Fees on payouts (2.5%) and bounties (5%) |
| What are margins? | ~95% gross margin per problem |
| When do we break even? | ~750 problems/month |
| What's the moat? | Soulbound reputation, network effects, civic partnerships |
| What's the risk? | Regulatory, adoption, manipulation |
| Why now? | Crypto infra ready, civic trust low, prediction markets mainstream |
| What's the upside? | Protocol for collective decision-making at scale |
