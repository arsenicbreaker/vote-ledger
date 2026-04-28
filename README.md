# Voting Program

A on-chain polling program built with [Anchor](https://www.anchor-lang.com/) on Solana. Demonstrates how to use PDAs to store structured state, pass typed arguments through instructions, and enforce time-gated access rules.

## Overview

Anyone can create a poll with a name, description, and a voting window. The poll creator then adds candidate options. During the voting window any wallet can cast a vote for a candidate; votes outside the window are rejected on-chain.

## Program ID

```
65KHV8cXwJ8apTKMqnpSdhdHkHhRySatgKMwnxm6C3gG
```

## Prerequisites

- [Rust](https://rustup.rs/)
- [Solana CLI](https://solana.com/developers/guides/getstarted/setup-local-development)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) v1.0.0-rc.2
- [Node.js](https://nodejs.org/) + [Yarn](https://yarnpkg.com/)
