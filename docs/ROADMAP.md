# Komon Roadmap

## Philosophy

This roadmap is opinionated:
- **Ship early, iterate fast** — Get to users quickly, learn from reality
- **Depth over breadth** — Nail one city before expanding
- **Mechanism first** — The core loop matters more than features
- **Decentralize gradually** — Earn trust before distributing control

Dates are directional. We'll move faster or slower based on what we learn.

---

## Phase 0: Foundation (Complete)

**Goal:** Prove the concept is buildable

### Delivered
- [x] Solana programs designed (ProblemRegistry, DirectionMarket, Treasury, Reputation)
- [x] Smart contract code written
- [x] Next.js frontend scaffolded
- [x] Prisma schema defined
- [x] AI analysis integration (Claude)
- [x] Documentation (Philosophy, Mechanism, Economics, etc.)
- [x] **Dual-mode architecture** (shared core + civic/creator framing layers)
- [x] **Creator mode implementation** (Vitalik's creator coin model)
- [x] Core programs: SubjectRegistry, MarketEngine, ReputationCore
- [x] Framing layers: Civic (Problem/Direction), Creator (DAO voting + burn)

### Status
Code exists. All programs compile successfully. Dual-mode architecture supports both civic problem-solving and creator curation. Frontend runs with mock data.

---

## Phase 1: MVP (Month 1-2)

**Goal:** Working product with real transactions on devnet

### Core Features

| Feature | Priority | Status |
|---------|----------|--------|
| User accounts (email + invisible wallet) | P0 | Planned |
| Post problem with location/category | P0 | Planned |
| Propose direction with description | P0 | Planned |
| AI feasibility analysis | P0 | Built |
| Stake YES/NO on directions | P0 | Planned |
| Basic payout on resolution | P0 | Planned |
| Authority-based verification | P0 | Planned |
| Reputation tracking (XP, level) | P1 | Planned |
| Leaderboard | P1 | Built (mock) |
| Problem/direction browsing | P1 | Built (mock) |

### Technical

| Task | Priority | Status |
|------|----------|--------|
| Deploy programs to Solana devnet | P0 | Blocked (toolchain) |
| Connect frontend to real database | P0 | Planned |
| Wire API routes to Solana transactions | P0 | Planned |
| Implement wallet abstraction | P0 | Planned |
| Add credit card → USDC on-ramp | P1 | Planned |
| Basic error handling and loading states | P1 | Planned |

### Milestones

- [ ] First problem created on devnet
- [ ] First direction proposed
- [ ] First stake placed
- [ ] First resolution verified
- [ ] First payout distributed

### Exit Criteria

- 10 problems posted by team/testers
- End-to-end flow works without manual intervention
- Can explain the product in 60 seconds

---

## Phase 2: Private Beta (Month 3-4)

**Goal:** Real users solving real problems in one neighborhood

### Features

| Feature | Priority |
|---------|----------|
| Bounty posting and funding | P0 |
| Problem success criteria editor | P0 |
| Before/after photo evidence | P0 |
| Notification system (email) | P0 |
| User profiles with stats | P1 |
| Problem filtering (category, location, status) | P1 |
| Direction comparison view | P1 |
| Basic mobile responsiveness | P1 |
| Share problem to social media | P2 |

### Operations

| Task | Priority |
|------|----------|
| Identify target neighborhood | P0 |
| Recruit 10 beta users (civic-minded) | P0 |
| Seed 5-10 real problems | P0 |
| Manual verification process documented | P0 |
| User feedback collection system | P1 |
| Weekly community check-ins | P1 |

### Milestones

- [ ] 50 registered users
- [ ] 20 problems posted
- [ ] 100 stakes placed
- [ ] 3 problems verified as resolved
- [ ] $500+ total volume

### Exit Criteria

- At least one problem solved "because of" Komon (user attribution)
- Net Promoter Score > 30
- Retention: 40% of users active after 30 days

---

## Phase 3: Public Beta (Month 5-8)

**Goal:** Prove the model works at city scale

### Features

| Feature | Priority |
|---------|----------|
| Committee-based verification (3-of-5) | P0 |
| Dispute/appeal process | P0 |
| Verifier training and onboarding | P0 |
| Problem categories with defaults | P1 |
| Direction templates | P1 |
| Reputation badges and achievements | P1 |
| Push notifications (mobile) | P1 |
| API for developers (read-only) | P2 |
| Embed widget for external sites | P2 |
| Dark mode | P2 |

### Growth

| Task | Priority |
|------|----------|
| Expand to full city (all neighborhoods) | P0 |
| Partner with 2-3 neighborhood associations | P0 |
| Local press coverage | P1 |
| Civic champion program (power users) | P1 |
| Referral incentives | P2 |

### Milestones

- [ ] 500 registered users
- [ ] 200 problems posted
- [ ] 1,000 stakes placed
- [ ] 50 problems resolved
- [ ] $10,000+ total volume
- [ ] 20 active verifiers

### Exit Criteria

- Self-sustaining activity (problems posted without team seeding)
- Verification committee functioning without intervention
- At least one "notable" resolution (press-worthy)

---

## Phase 4: Growth (Month 9-12)

**Goal:** Multi-city expansion, early revenue

### Features

| Feature | Priority |
|---------|----------|
| Multi-city support | P0 |
| City-specific leaderboards | P0 |
| Advanced analytics dashboard | P1 |
| API for write operations | P1 |
| Premium features (API access, analytics) | P1 |
| Governance proposals (parameter changes) | P2 |
| Mobile app (React Native) | P2 |
| Localization (Spanish, Japanese) | P2 |

### Expansion

| Task | Priority |
|------|----------|
| Launch in 2 additional cities | P0 |
| Document city launch playbook | P0 |
| Civic tech partnerships | P1 |
| 311 integration pilot | P1 |
| Academic research partnerships | P2 |

### Milestones

- [ ] 3 cities active
- [ ] 5,000 registered users
- [ ] 1,000 problems posted
- [ ] 200 problems resolved
- [ ] $100,000+ total volume
- [ ] First paying customer (API/premium)
- [ ] Breakeven on operational costs

### Exit Criteria

- Repeatable city launch process (< 2 weeks to meaningful activity)
- Revenue covers infrastructure costs
- User acquisition cost declining

---

## Phase 5: Scale (Year 2)

**Goal:** Become essential civic infrastructure

### Features

| Feature | Priority |
|---------|----------|
| Governance token or reputation-weighted voting | P1 |
| Full decentralized verification | P1 |
| Cross-city problem patterns | P1 |
| Government dashboard (read-only) | P1 |
| Insurance/risk data products | P2 |
| Prediction market for policy outcomes | P2 |

### Expansion

| Task | Priority |
|------|----------|
| 10+ cities | P0 |
| International expansion (1 country) | P1 |
| Government pilot partnership | P1 |
| Enterprise/institutional sales | P1 |

### Milestones

- [ ] 50,000 registered users
- [ ] 10,000 problems posted
- [ ] 2,000 problems resolved
- [ ] $1M+ total volume
- [ ] $500K+ annual revenue
- [ ] Government partnership signed

---

## Feature Parking Lot

Ideas we like but aren't prioritizing yet:

| Feature | Why Not Now |
|---------|-------------|
| Token-gated communities | Premature, adds complexity |
| NFT receipts for resolutions | Nice-to-have, not core |
| Quadratic funding for bounties | Need more volume first |
| AI-generated directions | Want human judgment first |
| Real-time video evidence | Mobile app prerequisite |
| Hardware oracle integration | IoT complexity |
| Prediction markets on policy | Regulatory considerations |
| Anonymous participation | Identity needed for reputation |
| ~~Creator coin model~~ | ~~Completed: Vitalik model implemented~~ |

### Recently Implemented (Moved from Parking Lot)

| Feature | Status |
|---------|--------|
| Creator mode (Vitalik model) | Implemented - DAO voting, burn mechanism, scout predictions |
| Shared core architecture | Implemented - SubjectRegistry, MarketEngine, ReputationCore |
| Dual framing layers | Implemented - Civic + Creator modes share same core |

---

## Technical Debt Priorities

### Must Address by Phase 3
- [ ] Comprehensive error handling
- [ ] Rate limiting and abuse prevention
- [ ] Database indexing optimization
- [ ] Proper logging and monitoring
- [ ] Automated testing (>70% coverage)

### Must Address by Phase 4
- [ ] Smart contract audit
- [ ] Load testing for scale
- [ ] Disaster recovery plan
- [ ] GDPR/privacy compliance
- [ ] Accessibility (WCAG 2.1)

---

## Risk-Adjusted Timeline

### Optimistic Path
Everything works. Users love it. Growth is organic.
- MVP: Month 1
- Private Beta: Month 2
- Public Beta: Month 4
- Growth: Month 7
- Scale: Month 12

### Realistic Path
Some things break. Iteration required. Moderate traction.
- MVP: Month 2
- Private Beta: Month 4
- Public Beta: Month 6
- Growth: Month 10
- Scale: Month 18

### Pessimistic Path
Major pivots needed. Slow adoption. Regulatory hurdles.
- MVP: Month 3
- Private Beta: Month 6
- Public Beta: Month 10
- Growth: Month 18
- Scale: Month 30+

We plan for realistic, hope for optimistic, prepare for pessimistic.

---

## Decision Points

### Phase 1 → 2
**Question:** Do people understand and use the core mechanism?
- If yes: Proceed to private beta
- If no: Simplify UX, improve onboarding, re-test

### Phase 2 → 3
**Question:** Are problems actually getting solved?
- If yes: Expand scope
- If no: Investigate why, adjust mechanism

### Phase 3 → 4
**Question:** Is there organic growth beyond seeded users?
- If yes: Invest in scale
- If no: Focus on retention and activation

### Phase 4 → 5
**Question:** Is this a sustainable business?
- If yes: Decentralize and scale
- If no: Pivot, partner, or wind down gracefully

---

## What Success Looks Like

### End of Year 1
- One city where Komon is "known" as a way to get things fixed
- 1,000+ problems posted organically
- 100+ problems resolved with clear attribution
- Community of engaged civic problem-solvers
- Sustainable unit economics

### End of Year 2
- Multiple cities with active communities
- Government partnerships interested in the signal
- Revenue from premium/institutional products
- Path to decentralized governance clear
- Referenced in civic tech discussions

### Long Term
- Default infrastructure for civic problem-solving
- "Post it on Komon" as common advice
- Data informs city budgets and priorities
- Protocol governed by the community
- Scaled globally with local adaptation
