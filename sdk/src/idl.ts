import { Idl } from "@coral-xyz/anchor";

/**
 * IDL for the Market Engine program.
 *
 * Manually constructed from programs/market-engine/src/lib.rs to replace
 * the minimal stub that had empty instructions/accounts/types arrays.
 *
 * Program ID: 5tFcB5fM3f19PB2CCqjBKvaNFqXWKgNjMALWknG291Qq
 */
export const MARKET_ENGINE_IDL: Idl = {
  address: "5tFcB5fM3f19PB2CCqjBKvaNFqXWKgNjMALWknG291Qq",
  metadata: {
    name: "market_engine",
    version: "0.1.0",
    spec: "0.1.0",
  },
  instructions: [
    {
      name: "initialize",
      discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
      docs: ["Initialize market engine config"],
      accounts: [
        {
          name: "config",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 99, 111, 110, 102, 105,
                  103,
                ],
              },
            ],
          },
        },
        {
          name: "authority",
          writable: true,
          signer: true,
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "fee_rate",
          type: "u16",
        },
        {
          name: "burn_enabled",
          type: "bool",
        },
        {
          name: "burn_rate_bps",
          type: "u16",
        },
      ],
    },
    {
      name: "create_market",
      discriminator: [132, 33, 218, 65, 177, 201, 57, 108],
      docs: ["Create a new market on a subject"],
      accounts: [
        {
          name: "config",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 99, 111, 110, 102, 105,
                  103,
                ],
              },
            ],
          },
        },
        {
          name: "subject",
          docs: ["The subject this market is for"],
        },
        {
          name: "market",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [109, 97, 114, 107, 101, 116],
              },
              {
                kind: "account",
                path: "subject",
              },
              {
                kind: "account",
                path: "config.market_count",
                account: "MarketConfig",
              },
            ],
          },
        },
        {
          name: "yes_mint",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [121, 101, 115, 95, 109, 105, 110, 116],
              },
              {
                kind: "account",
                path: "market",
              },
            ],
          },
        },
        {
          name: "no_mint",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [110, 111, 95, 109, 105, 110, 116],
              },
              {
                kind: "account",
                path: "market",
              },
            ],
          },
        },
        {
          name: "creator",
          writable: true,
          signer: true,
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "description",
          type: "string",
        },
        {
          name: "analysis",
          type: "string",
        },
      ],
    },
    {
      name: "stake",
      discriminator: [206, 176, 202, 18, 200, 209, 179, 108],
      docs: ["Stake on market outcome (buy YES or NO tokens)"],
      accounts: [
        {
          name: "config",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 99, 111, 110, 102, 105,
                  103,
                ],
              },
            ],
          },
        },
        {
          name: "market",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [109, 97, 114, 107, 101, 116],
              },
              {
                kind: "account",
                path: "market.subject",
                account: "Market",
              },
              {
                kind: "account",
                path: "market.id",
                account: "Market",
              },
            ],
          },
        },
        {
          name: "market_vault",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 118, 97, 117, 108, 116,
                ],
              },
              {
                kind: "account",
                path: "market",
              },
            ],
          },
        },
        {
          name: "yes_mint",
          writable: true,
        },
        {
          name: "no_mint",
          writable: true,
        },
        {
          name: "staker_tokens",
          writable: true,
        },
        {
          name: "staker_yes_tokens",
          writable: true,
        },
        {
          name: "staker_no_tokens",
          writable: true,
        },
        {
          name: "user_position",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  117, 115, 101, 114, 95, 112, 111, 115, 105, 116, 105, 111,
                  110,
                ],
              },
              {
                kind: "account",
                path: "market",
              },
              {
                kind: "account",
                path: "staker",
              },
            ],
          },
        },
        {
          name: "stake_mint",
        },
        {
          name: "staker",
          writable: true,
          signer: true,
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "outcome",
          type: {
            defined: {
              name: "Outcome",
            },
          },
        },
      ],
    },
    {
      name: "unstake",
      discriminator: [90, 95, 107, 42, 205, 124, 50, 225],
      docs: ["Unstake (sell tokens back before settlement)"],
      accounts: [
        {
          name: "market",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [109, 97, 114, 107, 101, 116],
              },
              {
                kind: "account",
                path: "market.subject",
                account: "Market",
              },
              {
                kind: "account",
                path: "market.id",
                account: "Market",
              },
            ],
          },
        },
        {
          name: "market_vault",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 118, 97, 117, 108, 116,
                ],
              },
              {
                kind: "account",
                path: "market",
              },
            ],
          },
        },
        {
          name: "yes_mint",
          writable: true,
        },
        {
          name: "no_mint",
          writable: true,
        },
        {
          name: "staker_tokens",
          writable: true,
        },
        {
          name: "staker_yes_tokens",
          writable: true,
        },
        {
          name: "staker_no_tokens",
          writable: true,
        },
        {
          name: "staker",
          writable: true,
          signer: true,
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
      ],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "outcome",
          type: {
            defined: {
              name: "Outcome",
            },
          },
        },
      ],
    },
    {
      name: "settle_market",
      discriminator: [172, 112, 191, 140, 14, 60, 180, 195],
      docs: ["Settle market after subject resolution"],
      accounts: [
        {
          name: "config",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 99, 111, 110, 102, 105,
                  103,
                ],
              },
            ],
          },
        },
        {
          name: "market",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [109, 97, 114, 107, 101, 116],
              },
              {
                kind: "account",
                path: "market.subject",
                account: "Market",
              },
              {
                kind: "account",
                path: "market.id",
                account: "Market",
              },
            ],
          },
        },
        {
          name: "authority",
          signer: true,
        },
      ],
      args: [
        {
          name: "outcome",
          type: {
            defined: {
              name: "Outcome",
            },
          },
        },
      ],
    },
    {
      name: "claim_rewards",
      discriminator: [4, 144, 132, 71, 116, 23, 151, 80],
      docs: [
        "Claim rewards after settlement",
        "Includes burn mechanism for Creator mode",
      ],
      accounts: [
        {
          name: "config",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 99, 111, 110, 102, 105,
                  103,
                ],
              },
            ],
          },
        },
        {
          name: "market",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [109, 97, 114, 107, 101, 116],
              },
              {
                kind: "account",
                path: "market.subject",
                account: "Market",
              },
              {
                kind: "account",
                path: "market.id",
                account: "Market",
              },
            ],
          },
        },
        {
          name: "market_vault",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  109, 97, 114, 107, 101, 116, 95, 118, 97, 117, 108, 116,
                ],
              },
              {
                kind: "account",
                path: "market",
              },
            ],
          },
        },
        {
          name: "user_position",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  117, 115, 101, 114, 95, 112, 111, 115, 105, 116, 105, 111,
                  110,
                ],
              },
              {
                kind: "account",
                path: "market",
              },
              {
                kind: "account",
                path: "staker",
              },
            ],
          },
        },
        {
          name: "staker_tokens",
          writable: true,
        },
        {
          name: "staker_yes_tokens",
        },
        {
          name: "staker_no_tokens",
        },
        {
          name: "yes_mint",
        },
        {
          name: "no_mint",
        },
        {
          name: "burn_treasury",
          writable: true,
          docs: ["Burn treasury (receives burn amount in Creator mode)"],
        },
        {
          name: "staker",
          writable: true,
          signer: true,
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: "MarketConfig",
      discriminator: [223, 245, 109, 209, 44, 92, 26, 81],
    },
    {
      name: "Market",
      discriminator: [219, 190, 213, 55, 0, 227, 198, 154],
    },
    {
      name: "UserPosition",
      discriminator: [251, 248, 209, 245, 83, 234, 17, 27],
    },
  ],
  events: [
    {
      name: "MarketCreated",
      discriminator: [12, 225, 183, 130, 85, 60, 253, 106],
    },
    {
      name: "StakePlaced",
      discriminator: [189, 238, 73, 59, 148, 104, 210, 35],
    },
    {
      name: "StakeWithdrawn",
      discriminator: [19, 118, 113, 141, 218, 162, 152, 63],
    },
    {
      name: "MarketSettled",
      discriminator: [167, 33, 83, 152, 193, 87, 237, 91],
    },
    {
      name: "RewardsClaimed",
      discriminator: [143, 98, 80, 110, 69, 58, 236, 150],
    },
  ],
  errors: [
    {
      code: 6000,
      name: "DescriptionTooLong",
      msg: "Description too long",
    },
    {
      code: 6001,
      name: "AnalysisTooLong",
      msg: "Analysis too long",
    },
    {
      code: 6002,
      name: "InvalidAmount",
      msg: "Invalid amount",
    },
    {
      code: 6003,
      name: "MarketSettled",
      msg: "Market already settled",
    },
    {
      code: 6004,
      name: "AlreadySettled",
      msg: "Market already settled",
    },
    {
      code: 6005,
      name: "MarketNotSettled",
      msg: "Market not yet settled",
    },
    {
      code: 6006,
      name: "AlreadyClaimed",
      msg: "Already claimed",
    },
    {
      code: 6007,
      name: "Unauthorized",
      msg: "Unauthorized",
    },
    {
      code: 6008,
      name: "ArithmeticOverflow",
      msg: "Arithmetic overflow",
    },
  ],
  types: [
    {
      name: "MarketConfig",
      type: {
        kind: "struct",
        fields: [
          {
            name: "authority",
            type: "pubkey",
          },
          {
            name: "market_count",
            type: "u64",
          },
          {
            name: "fee_rate",
            type: "u16",
          },
          {
            name: "burn_enabled",
            type: "bool",
          },
          {
            name: "burn_rate_bps",
            type: "u16",
          },
          {
            name: "total_volume",
            type: "u64",
          },
          {
            name: "total_burned",
            type: "u64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "Market",
      type: {
        kind: "struct",
        fields: [
          {
            name: "id",
            type: "u64",
          },
          {
            name: "subject",
            type: "pubkey",
          },
          {
            name: "creator",
            type: "pubkey",
          },
          {
            name: "description",
            type: "string",
          },
          {
            name: "analysis",
            type: "string",
          },
          {
            name: "yes_mint",
            type: "pubkey",
          },
          {
            name: "no_mint",
            type: "pubkey",
          },
          {
            name: "yes_supply",
            type: "u64",
          },
          {
            name: "no_supply",
            type: "u64",
          },
          {
            name: "liquidity",
            type: "u64",
          },
          {
            name: "is_settled",
            type: "bool",
          },
          {
            name: "outcome",
            type: {
              option: {
                defined: {
                  name: "Outcome",
                },
              },
            },
          },
          {
            name: "created_at",
            type: "i64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "UserPosition",
      type: {
        kind: "struct",
        fields: [
          {
            name: "user",
            type: "pubkey",
          },
          {
            name: "market",
            type: "pubkey",
          },
          {
            name: "claimed",
            type: "bool",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "Outcome",
      type: {
        kind: "enum",
        variants: [
          {
            name: "Yes",
          },
          {
            name: "No",
          },
        ],
      },
    },
    {
      name: "MarketCreated",
      type: {
        kind: "struct",
        fields: [
          {
            name: "market_id",
            type: "u64",
          },
          {
            name: "subject",
            type: "pubkey",
          },
          {
            name: "creator",
            type: "pubkey",
          },
          {
            name: "description",
            type: "string",
          },
        ],
      },
    },
    {
      name: "StakePlaced",
      type: {
        kind: "struct",
        fields: [
          {
            name: "market_id",
            type: "u64",
          },
          {
            name: "staker",
            type: "pubkey",
          },
          {
            name: "amount",
            type: "u64",
          },
          {
            name: "outcome",
            type: {
              defined: {
                name: "Outcome",
              },
            },
          },
          {
            name: "tokens_minted",
            type: "u64",
          },
        ],
      },
    },
    {
      name: "StakeWithdrawn",
      type: {
        kind: "struct",
        fields: [
          {
            name: "market_id",
            type: "u64",
          },
          {
            name: "staker",
            type: "pubkey",
          },
          {
            name: "amount",
            type: "u64",
          },
          {
            name: "outcome",
            type: {
              defined: {
                name: "Outcome",
              },
            },
          },
        ],
      },
    },
    {
      name: "MarketSettled",
      type: {
        kind: "struct",
        fields: [
          {
            name: "market_id",
            type: "u64",
          },
          {
            name: "outcome",
            type: {
              defined: {
                name: "Outcome",
              },
            },
          },
          {
            name: "total_liquidity",
            type: "u64",
          },
        ],
      },
    },
    {
      name: "RewardsClaimed",
      type: {
        kind: "struct",
        fields: [
          {
            name: "market_id",
            type: "u64",
          },
          {
            name: "staker",
            type: "pubkey",
          },
          {
            name: "gross_payout",
            type: "u64",
          },
          {
            name: "burn_amount",
            type: "u64",
          },
          {
            name: "net_payout",
            type: "u64",
          },
        ],
      },
    },
  ],
} as unknown as Idl;
