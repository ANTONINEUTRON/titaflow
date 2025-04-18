/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/tita_flow.json`.
 */
export type TitaFlow = {
  "address": "2coGXpnEG9qJMzKZCxvgfSznWoDzVFazugp4VMSvbUYk",
  "metadata": {
    "name": "titaFlow",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createFlow",
      "discriminator": [
        139,
        104,
        255,
        32,
        61,
        236,
        41,
        31
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "flowAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  116,
                  97,
                  45,
                  102,
                  108,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "arg",
                "path": "flowId"
              }
            ]
          }
        },
        {
          "name": "flowTokenAccount",
          "docs": [
            "Associated token account for the flow (escrow)"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  116,
                  97,
                  45,
                  102,
                  108,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "flowAccount"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "flowId",
          "type": "string"
        },
        {
          "name": "flowType",
          "type": "u8"
        },
        {
          "name": "goal",
          "type": "u64"
        },
        {
          "name": "startDate",
          "type": "i64"
        },
        {
          "name": "endDate",
          "type": {
            "option": "i64"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "flowAccount",
      "discriminator": [
        146,
        131,
        230,
        252,
        108,
        20,
        107,
        83
      ]
    }
  ],
  "events": [
    {
      "name": "flowCreatedEvent",
      "discriminator": [
        97,
        126,
        21,
        187,
        40,
        199,
        96,
        104
      ]
    },
    {
      "name": "ruleAddedEvent",
      "discriminator": [
        123,
        229,
        218,
        189,
        80,
        113,
        27,
        117
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidMerkleProof",
      "msg": "Rule is not satisfied"
    }
  ],
  "types": [
    {
      "name": "flowAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "flowId",
            "type": "string"
          },
          {
            "name": "flowType",
            "type": {
              "defined": {
                "name": "flowType"
              }
            }
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "goal",
            "type": "u64"
          },
          {
            "name": "raised",
            "type": "u64"
          },
          {
            "name": "balanceInTa",
            "type": "u64"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "startDate",
            "type": "i64"
          },
          {
            "name": "endDate",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "senderCount",
            "type": "u32"
          },
          {
            "name": "receiverCount",
            "type": "u32"
          },
          {
            "name": "ruleAccountsCounts",
            "type": "u8"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "flowStatus"
              }
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "flowCreatedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "flowId",
            "type": "string"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "goal",
            "type": "u64"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "startDate",
            "type": "i64"
          },
          {
            "name": "endDate",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "flowType",
            "type": {
              "defined": {
                "name": "flowType"
              }
            }
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "flowStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "draft"
          },
          {
            "name": "active"
          },
          {
            "name": "paused"
          },
          {
            "name": "completed"
          },
          {
            "name": "canceled"
          },
          {
            "name": "expired"
          },
          {
            "name": "underReview"
          }
        ]
      }
    },
    {
      "name": "flowType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "send"
          },
          {
            "name": "receive"
          }
        ]
      }
    },
    {
      "name": "ruleAddedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "flow",
            "type": "pubkey"
          },
          {
            "name": "rule",
            "type": "pubkey"
          },
          {
            "name": "ruleType",
            "type": "string"
          },
          {
            "name": "createdAt",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
