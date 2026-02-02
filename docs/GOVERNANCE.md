# Komon Governance

## Overview

Governance answers three questions:
1. **Who decides?** — How are decisions made and by whom?
2. **What can change?** — Which parameters and rules are modifiable?
3. **How do we get there?** — What's the path from centralized to decentralized?

Komon starts centralized (for speed and iteration) and progressively decentralizes as the protocol matures and trust is established.

---

## Governance Scope

### Protocol Parameters

| Parameter | Current | Governable | Notes |
|-----------|---------|------------|-------|
| Payout fee rate | 2.5% | Yes | Range: 0-10% |
| Bounty fee rate | 5% | Yes | Range: 0-15% |
| Unstake fee rate | 1% | Yes | Range: 0-5% |
| Minimum stake | $1 | Yes | Floor for participation |
| Maximum stake | $1000 | Yes | Risk management |
| Problem deadline range | 7-90 days | Yes | Min/max allowed |
| Verification threshold | 3-of-5 | Yes | M-of-N verifiers required |
| Reputation XP formula | See code | Yes | Level progression curve |

### Protocol Upgrades

| Change Type | Process | Authority |
|-------------|---------|-----------|
| Bug fixes | Immediate deploy | Core team |
| Parameter changes | 7-day notice | Core team → DAO |
| New features | Community discussion + vote | DAO |
| Breaking changes | 30-day notice + migration | DAO supermajority |
| Emergency pause | Immediate | Multisig (3-of-5) |

### Treasury Management

| Decision | Authority |
|----------|-----------|
| Fee collection | Automatic (protocol) |
| Operational expenses | Core team (with transparency) |
| Grants and bounties | DAO vote |
| Strategic reserves | DAO vote |

---

## Dispute Resolution

### Types of Disputes

**1. Outcome Disputes**
- "The problem wasn't actually solved"
- "The wrong direction was credited"
- "The verification was fraudulent"

**2. Conduct Disputes**
- "This user is manipulating markets"
- "This problem is spam/fake"
- "This direction is plagiarized"

**3. Technical Disputes**
- "The smart contract behaved unexpectedly"
- "My transaction failed but funds were taken"

### Resolution Process

```
┌─────────────────────────────────────────────────────────────────┐
│  LEVEL 1: Automated                                              │
│  - Clear-cut cases resolved by smart contract logic              │
│  - Example: Deadline passed, no resolution → NO stakers win      │
└──────────────────────────┬──────────────────────────────────────┘
                           │ Dispute filed
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  LEVEL 2: Verification Committee                                 │
│  - 5 randomly selected verifiers from high-reputation pool       │
│  - 3-of-5 must agree                                             │
│  - 48-hour resolution window                                     │
│  - Verifiers stake reputation on their decision                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │ Appeal filed (requires stake)
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  LEVEL 3: Community Arbitration                                  │
│  - Open to all users with reputation > Level 5                   │
│  - Stake-weighted voting                                         │
│  - 7-day voting period                                           │
│  - Supermajority (67%) required to overturn                      │
└──────────────────────────┬──────────────────────────────────────┘
                           │ Final appeal (rare, high stake)
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  LEVEL 4: Governance Council                                     │
│  - 7-member elected council                                      │
│  - Reserved for precedent-setting cases                          │
│  - Decision becomes binding protocol policy                      │
└─────────────────────────────────────────────────────────────────┘
```

### Dispute Costs

| Level | Filing Cost | Refund if Successful |
|-------|-------------|---------------------|
| Level 2 | $10 | 100% |
| Level 3 | $50 | 100% |
| Level 4 | $200 | 100% |

Filing costs prevent spam disputes. Full refund if the dispute is upheld.

---

## Path to Decentralization

### Phase 1: Benevolent Dictatorship (Now)

**Who decides:** Core team

**Rationale:** Speed of iteration, ability to fix mistakes, establish norms

**What's centralized:**
- All protocol parameters
- Outcome verification
- Dispute resolution
- Treasury management

**Accountability:**
- All decisions logged publicly
- Weekly transparency reports
- Community feedback channels

### Phase 2: Guided Decentralization (Month 6-12)

**Who decides:** Core team + Community input

**Changes:**
- Verification committee launched (high-rep users)
- Parameter changes require community discussion
- Treasury grants decided by community vote
- Core team retains veto for safety

**Milestones to proceed:**
- 1,000+ active users
- 100+ verified resolutions
- Stable verification process

### Phase 3: Progressive Decentralization (Year 2)

**Who decides:** DAO with core team guardrails

**Changes:**
- Governance token launched (or reputation-weighted voting)
- Protocol parameters controlled by DAO
- Core team veto limited to security issues
- Elected governance council

**Milestones to proceed:**
- 10,000+ active users
- Geographic diversity (10+ cities)
- No critical security incidents

### Phase 4: Full Decentralization (Year 3+)

**Who decides:** DAO

**Changes:**
- Core team becomes one voice among many
- Protocol upgrades require DAO approval
- Treasury fully controlled by DAO
- Emergency multisig remains for security

**Milestones to proceed:**
- Proven dispute resolution system
- Multiple independent verification providers
- Sustainable without core team intervention

---

## Governance Token (Future)

### If/When We Launch a Token

**Purpose:** Governance rights, not speculation

**Distribution (Indicative):**
| Allocation | Percentage | Vesting |
|------------|------------|---------|
| Community (retroactive users) | 40% | Immediate |
| Core team | 20% | 4-year vest, 1-year cliff |
| Treasury | 25% | DAO-controlled |
| Investors | 15% | 2-year vest |

**Voting Power:**
- 1 token = 1 vote for parameter changes
- Reputation multiplier for dispute resolution
- Quadratic voting for major decisions (reduces whale power)

**Why we might not launch a token:**
- Reputation-weighted governance may be sufficient
- Tokens attract speculators, not users
- Regulatory complexity
- If it's not needed, we won't do it

---

## Checks and Balances

### Against Core Team Capture

- All protocol changes logged on-chain
- Community can fork if trust is broken
- Progressive removal of special privileges
- Financial transparency (quarterly reports)

### Against Whale Capture

- Reputation matters, not just stake
- Quadratic voting for major decisions
- Maximum stake limits
- Geographic distribution requirements for council

### Against Verifier Collusion

- Random selection from large pool
- Verifiers don't know each other's votes until reveal
- Reputation staked on decisions
- Appeals process with broader participation

### Against Governance Attacks

- Time locks on major changes (7-30 days)
- Supermajority requirements for breaking changes
- Emergency pause capability
- Gradual parameter change limits (can't 10x fees overnight)

---

## Decision-Making Principles

**1. Defaults matter**
Most users won't vote. Defaults should be safe and reasonable. Changes should require active consent, not passive acceptance.

**2. Skin in the game**
Those affected by decisions should make them. Verifiers stake reputation. Voters stake tokens or reputation. No consequence-free opinions.

**3. Reversibility**
Prefer reversible decisions. If we're unsure, try it with a time limit. Irreversible changes require higher thresholds.

**4. Transparency**
All governance actions are public. Reasoning is documented. Dissent is recorded. No backroom deals.

**5. Subsidiarity**
Decisions should be made at the lowest effective level. Individual problems don't need protocol-wide votes. Only escalate what must be escalated.

---

## Current Governance Status

**As of launch:**

| Area | Authority | Process |
|------|-----------|---------|
| Protocol parameters | Core team | Internal discussion, public announcement |
| Outcome verification | Core team | Manual review |
| Disputes | Core team | Case-by-case |
| Treasury | Core team | Monthly transparency report |
| Roadmap | Core team | Community input welcome |

**How to participate now:**
- GitHub discussions for feature requests
- Discord for community feedback
- Public roadmap with voting on priorities
- Bug bounty program for security researchers

We're centralized today so we can be decentralized tomorrow. Trust is earned, then encoded.
