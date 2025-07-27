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
use tronic::trx;
use tronic::address::TronAddress;
use tronic::signer::LocalSigner;

// Connect to full node and prepare provider
let provider = GrpcProvider::new("https://api.trongrid.io".parse()?, Auth::None).await?;

// Construct a client with a signing backend
let client = Client::builder()
    .provider(provider)
    .signer(LocalSigner::rand())
    .build()?;

// Send TRX
let tx = client.send_trx(from, to, trx!(2 TRX)).await?;
```

## TODO

- [ ] Implement batching
- [ ] Unit & integration testing
- [ ] More examples
- [ ] Prepare documentation
