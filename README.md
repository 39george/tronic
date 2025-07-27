# 🦀 tronic

> A modular, type-safe, async-first Rust client for the Tron blockchain — inspired by Alloy and built for real-world smart contract interaction.

---

## ✨ Features

### Core Infrastructure
- 🧱 **Typed Smart Contract Calls** — Powered by `alloy-sol-types` macros for compile-time safety
- 📡 **Dual Protocol Support** — Both gRPC (via `tonic`) and JSON-RPC providers (WIP)
- 🔐 **Pluggable Signers** — Local, or custom async signing backends
- ⚡ **Async-First** — Built on Tokio for high-performance blockchain interaction

### Account Management
- 🔑 **Multi-Signature Support** — Complete workflow for threshold signatures
- 🏦 **Permission Management** — Modify account permissions programmatically
- ❄️ **Resource Delegation** — Handle bandwidth/energy freezing/unfreezing (WIP)

### Smart Contract Interaction
- 📝 **TRC-20** — Type-safe token transfers with `alloy`-style builders
- 📜 **Contract ABI Codegen** — Generate types from Solidity ABIs (need to implement a wrapper by hand currently)
- 🔍 **Event Filtering** — Rich event subscription and historical querying
- 🧮 **Transaction Estimation** — Precise energy & bandwidth calculation with fallback modes

### Advanced Transaction Features
- 🚦 **Transaction Batching** — Group multiple operations atomically (WIP)
- ⏱️ **Deadline Handling** — Automatic/manual transaction expiration management

---


## 🚀 Quickstart

```rust
use tronic::client::Client;
use tronic::client::pending::AutoSigning;
use tronic::domain::address::TronAddress;
use tronic::provider::grpc::GrpcProvider;
use tronic::signer::LocalSigner;
use tronic::trx;

// Construct a client with a signing backend
let client = Client::builder()
    .provider(
        // Build grpc provider
        GrpcProvider::new(
            "https://grpc.trongrid.io:50051".parse()?,
            tronic::client::Auth::None,
        )
        .await?,
    )
    .signer(LocalSigner::rand())
    .build();

// Send transaction
let txid = client
    .send_trx()
    .to(TronAddress::rand())
    .amount(trx!(1.0 TRX))
    .build::<AutoSigning>() // Uses automatic signing strategy
    .await?
    .broadcast(&())
    .await?;
```

## TODO

- [ ] Implement batching
- [ ] Unit & integration testing
- [ ] More examples
- [ ] Prepare documentation
