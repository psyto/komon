# Komon Verification

## The Core Problem

Prediction markets need oracles. Someone has to say "this happened" or "this didn't happen." In traditional markets, this is often a trusted authority or a decentralized oracle network reporting on objective data (prices, election results, sports scores).

Civic problems are harder:
- **Subjective:** Was the pothole "fixed" if it was patched but poorly?
- **Local:** No global data feed for "streetlight on Elm Street"
- **Nuanced:** Did the community petition "work" if the city responded but slowly?

Verification is the hardest part of Komon. Get it wrong, and the entire mechanism fails.

---

## Verification Principles

**1. Clear criteria upfront**
The definition of "solved" must be established when the problem is posted, not when it's resolved.

**2. Multiple independent verifiers**
No single point of failure or corruption. Require agreement among several parties.

**3. Skin in the game**
Verifiers must stake something—reputation, money, or both. Incorrect verification has consequences.

**4. Escalation path**
Initial verification can be challenged. Higher-stakes disputes get more scrutiny.

**5. Transparency**
All verification decisions and reasoning are public and auditable.

---

## Verification Flow

### Step 1: Problem Definition

When a problem is posted, the creator must specify:

```
Problem: Pothole on Main Street at Oak Avenue

Success Criteria:
- [ ] Pothole is filled with permanent material (not cold patch)
- [ ] Surface is level with surrounding road (±1 inch)
- [ ] Repair persists for at least 7 days

Evidence Required:
- Before photo with timestamp and location
- After photo with timestamp and location
- Date of repair

Deadline: 30 days from posting
```

**Why this matters:** Vague success criteria lead to disputes. "Fixed" means different things to different people. Explicit criteria create shared understanding.

### Step 2: Resolution Claim

When someone believes the problem is solved:

1. **Claimant** submits evidence (photos, documentation, witness statements)
2. **Claimant** identifies which direction(s) contributed to the resolution
3. **Verification window** opens (48-72 hours)

Anyone can submit a resolution claim—the problem creator, direction proposer, or any community member.

### Step 3: Initial Verification

**For MVP (Centralized):**
- Core team reviews evidence against success criteria
- Binary decision: Resolved or Not Resolved
- If resolved, winning direction(s) identified

**For V1 (Committee):**
- 5 verifiers randomly selected from qualified pool
- Each reviews evidence independently
- 3-of-5 must agree for resolution
- Verifiers don't see others' votes until all submitted

**Verifier Qualifications:**
- Reputation Level 5+
- No stake in the problem being verified
- Geographic proximity preferred (for local knowledge)
- History of accurate verifications

### Step 4: Challenge Period

After initial verification, a 24-hour challenge window opens.

**Who can challenge:**
- Any staker on the problem
- The problem creator
- Direction proposers

**Challenge requirements:**
- Stake $10 (refunded if challenge succeeds)
- Provide counter-evidence or reasoning

### Step 5: Escalated Verification (If Challenged)

```
Challenge Filed
      │
      ▼
┌─────────────────────────────────────┐
│  Expanded Committee (9 verifiers)   │
│  - Original 5 recused               │
│  - 9 new verifiers selected         │
│  - 6-of-9 agreement required        │
│  - 72-hour window                   │
└──────────────────┬──────────────────┘
                   │
        ┌──────────┴──────────┐
        ▼                     ▼
   Upheld                 Overturned
        │                     │
        ▼                     ▼
   Original result      New result
   stands               implemented
        │                     │
        ▼                     ▼
   Challenger loses     Challenger refunded
   stake                Original verifiers
                        lose reputation
```

### Step 6: Final Appeal (Rare)

For precedent-setting cases or high-value disputes:

- Requires $200 stake
- Goes to Governance Council (7 elected members)
- Decision becomes binding precedent
- 7-day deliberation period

---

## Verifier Incentives

### Why Verify?

**Reputation rewards:**
- Successful verification: +10 XP
- Consistent accuracy: "Trusted Verifier" badge
- High accuracy unlocks higher-stakes verifications

**Financial rewards (Future):**
- Share of verification fees
- Priority access to new features
- Potential token allocation

### Why Verify Honestly?

**Reputation costs:**
- Overturned decision: -50 XP
- Pattern of bad decisions: Verification privileges revoked
- Egregious misconduct: Account flagged

**Financial costs (Future):**
- Verification stake slashed
- Removed from fee-sharing pool

### Collusion Prevention

**Random selection:** Verifiers don't know who else is on the committee until voting closes.

**Commit-reveal:** Verifiers submit hashed votes first, reveal later. Can't change vote after seeing others.

**Geographic distribution:** For local problems, verifiers from the area are weighted higher but never form a majority.

**Stake requirement:** Verifiers must have skin in the game (reputation or future financial stake).

---

## Edge Cases

### Partial Resolution

**Scenario:** Pothole was fixed, but only after the deadline.

**Approach:**
- If resolved within 7 days of deadline: Verifiers decide if "close enough"
- If resolved much later: Not resolved (deadline matters)
- Success criteria should specify timing requirements

### Multiple Contributing Directions

**Scenario:** Direction A proposed a petition, Direction B proposed contacting the city directly. The creator did both. Problem got solved.

**Approach:**
- Verifiers can credit multiple directions
- Stakers on any credited direction win proportionally
- Up to 3 directions can share credit

### Disputed Evidence

**Scenario:** Resolution photo is claimed to be from a different location.

**Approach:**
- Require metadata (GPS, timestamp) when possible
- Verifiers weigh evidence quality
- Challenge process exists for fraud claims
- Persistent fraud leads to account termination

### External Resolution

**Scenario:** The city fixed the pothole through normal processes, not because of any Komon direction.

**Approach:**
- If the problem is solved, it's solved—regardless of cause
- Directions that predicted "city will handle it eventually" might win
- The mechanism rewards correct judgment about what would happen

### No Resolution

**Scenario:** Deadline passes, problem unsolved.

**Approach:**
- NO stakers on all directions win
- They correctly predicted nothing would work
- Bounty returns to creator (minus fees)

---

## Evidence Standards

### Required Evidence

| Problem Type | Minimum Evidence |
|--------------|------------------|
| Physical infrastructure | Before/after photos with location metadata |
| Safety issues | Official report or documented incident reduction |
| Environmental | Photos + any official monitoring data |
| Process/policy | Documentation of policy change or official response |

### Evidence Quality Tiers

**Tier 1 (Strongest):**
- Official government confirmation
- Third-party inspection report
- Multiple independent photo sources

**Tier 2 (Strong):**
- Timestamped, geotagged photos
- Local news coverage
- Multiple witness statements

**Tier 3 (Acceptable):**
- Single photo source with metadata
- Creator confirmation
- Reasonable inference from circumstances

**Tier 4 (Weak, requires corroboration):**
- Photos without metadata
- Single unverified claim
- Circumstantial evidence only

### Verification Checklist

Verifiers should confirm:

- [ ] Evidence matches the specific location
- [ ] Evidence is dated after problem was posted
- [ ] Success criteria are clearly met (not just "improved")
- [ ] No obvious signs of manipulation or fraud
- [ ] If challenged, counter-evidence is weaker than primary evidence

---

## Verification Economics

### Costs

| Activity | Who Pays | Amount |
|----------|----------|--------|
| Initial verification | Protocol (from fees) | ~$2 in verifier rewards |
| Challenge | Challenger | $10 (refundable) |
| Expanded verification | Protocol | ~$5 in verifier rewards |
| Final appeal | Appellant | $200 (refundable) |

### Revenue Allocation

From the 2.5% payout fee:
- 1.5% → Protocol treasury
- 0.5% → Verifier reward pool
- 0.5% → Dispute resolution reserve

---

## Future Improvements

### Photographic Proof Protocol

- Dedicated mobile app for evidence capture
- Automatic metadata embedding (GPS, timestamp, device ID)
- Tamper-evident hashing
- Integration with Street View for location verification

### Reputation-Based Verification

- High-reputation users can verify smaller problems solo
- Graduated system based on problem value and complexity
- Reduces overhead for obvious cases

### AI-Assisted Verification

- Computer vision for before/after comparison
- Anomaly detection in evidence patterns
- Not decision-making, but flagging for human review

### Decentralized Verifier Networks

- Specialized verifiers for different problem categories
- Cross-city verifier exchange
- Verifier DAOs with shared standards

---

## Summary

Verification is where Komon's mechanism meets reality. The approach:

1. **Define clearly** — Success criteria upfront, not after the fact
2. **Verify independently** — Multiple verifiers, no single point of failure
3. **Incentivize honesty** — Reputation and future financial stakes
4. **Allow appeals** — Escalation path for disputes
5. **Stay transparent** — All decisions public and auditable

The goal is a system where:
- Honest verification is the easy path
- Cheating is expensive and visible
- Edge cases have clear resolution processes
- Trust is earned through consistent accuracy

Verification will evolve as Komon scales. We start simple (centralized review), add complexity as needed (committees, appeals), and decentralize as trust is established.
