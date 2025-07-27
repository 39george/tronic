# 🦀 tronic

> A modular, type-safe, async-first Rust client for the Tron blockchain — inspired by Alloy and built for real-world smart contract interaction.

---

## ✨ Features

- 🧱 **Typed Smart Contract Calls** — Powered by `alloy-sol-types` macros
- 📡 **gRPC Provider** — Built on `tonic` for high-performance communication with full nodes
- 🔐 **Signer Abstraction** — Supports pluggable async signing backends
- 🔗 **TRC-20 & Native TRX Support** — Transfer tokens and interact with contracts safely
- ⚙️ **Extendable** — Build your own providers, signers, and integrations

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
