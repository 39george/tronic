# tronic

[![Crates.io](https://img.shields.io/crates/v/tronic)](https://crates.io/crates/tronic)
[![docs](https://docs.rs/tronic/badge.svg)](https://docs.rs/tronic/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/39george/tronic/blob/main/LICENSE)

Async Rust client for Tron, talking to java-tron nodes over gRPC. Typed
transactions, TRC-20 via `alloy-sol-types`, resource estimation before
broadcast, multisig and external signers.

The crate is pre-1.0: minor versions may break the API (see the changelog
before bumping).

```toml
[dependencies]
tronic = "0.8"
```

Features:

- `tonic-tls` — TLS for `https://` endpoints. Without it the provider refuses
  to connect to https URLs.
- `mock-provider` — `MockProvider`, a `TronProvider` that returns errors for
  every call. Useful as a stub in tests.

## Sending TRX

```rust
use tronic::client::Client;
use tronic::client::pending::AutoSigning;
use tronic::domain::address::TronAddress;
use tronic::provider::grpc::GrpcProvider;
use tronic::signer::LocalSigner;
use tronic::trx;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = GrpcProvider::builder()
        .connect("http://grpc.nile.trongrid.io:50051")
        .await?;

    let key = hex::decode(std::env::var("PRIVATE_KEY")?)?;
    let client = Client::builder()
        .provider(provider)
        .signer(LocalSigner::from_bytes(&key)?)
        .build();

    let to: TronAddress = "TNPeeaaFB7K9cmo4uQpcU32zGK8G1NYqeL".parse()?;

    let txid = client
        .send_trx()
        .to(to)
        .amount(trx!(1.0 TRX))
        .build::<AutoSigning>()
        .await?
        .broadcast(&())
        .await?;

    println!("sent: {txid:?}");
    Ok(())
}
```

`build()` fetches the latest block for TaPoS, estimates energy and sets
`fee_limit` (estimate × 1.5, or a fixed fallback for calls that can't be
estimated). `broadcast()` signs, checks that the account has enough
bandwidth, energy and TRX, and only then sends. If something is missing you
get `Error::InsufficientResources` with the details instead of a failed
transaction on chain.

To wait for the transaction to land in a block, use
`broadcast_get_receipt(&(), confirmations)` instead of `broadcast`.

`trx!` accepts `trx!(1.5 TRX)` and `trx!(1_500_000 SUN)`.

## Connecting

```rust
use secrecy::SecretString;
use std::time::Duration;
use tronic::RateLimit;
use tronic::client::Auth;

let provider = GrpcProvider::builder()
    .auth(Auth::Bearer {
        name: "TRON-PRO-API-KEY".into(),
        secret: SecretString::from(api_key),
    })
    .rate_limit(RateLimit { limit: 15, duration: Duration::from_secs(1) })
    .connect_timeout(Duration::from_secs(5))
    .connect("http://grpc.trongrid.io:50051")
    .await?;
```

Retryable gRPC errors (`Unavailable`, `DeadlineExceeded`,
`ResourceExhausted`, …) are retried 3 times with backoff. Turn this off with
`.retry_enabled(false)`.

If you need the number of the last solidified block, pass the node's
WalletSolidity endpoint with `.solidity_uri(...)`. On a self-hosted FullNode
that is `rpc.solidityPort` (50061 by default) — the service is part of the
FullNode, it just has to be exposed. Without `solidity_uri` the number is
taken from `GetNodeInfo`.

## TRC-20

```rust
use tronic::contracts::token::usdt::Usdt;
use tronic::contracts::trc20::{Trc20Calls, Trc20Contract};

let usdt = Trc20Contract::<Usdt>::new("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".parse()?);

// owner defaults to the signer's address
let balance = client.trc20_balance_of().contract(usdt.clone()).get().await?;

let txid = client
    .trc20_transfer()
    .contract(usdt)
    .to(to)
    .amount(Usdt::from_decimal(10.0)?)
    .build::<AutoSigning>()
    .await?
    .broadcast(&())
    .await?;
```

`Usdt` is a 6-decimals token type. For another token implement the `Token`
trait (`symbol()`, `decimals()`, conversion to/from `U256`).

For arbitrary read-only calls there is `client.read_contract(call)`; for
deployment, `client.create_contract(artifact_json)`.

## Estimating before sending

```rust
let ptx = client
    .send_trx()
    .to(to)
    .amount(trx!(10.0 TRX))
    .build::<AutoSigning>()
    .await?;

let state = ptx.estimate_transaction().await?;
println!("will consume: {:?}", state.will_consume);
if let Some(missing) = &state.insufficient {
    println!("missing: {:?}, suggested top-up: {:?}",
        missing.missing, missing.suggested_trx_topup);
}
```

The estimate accounts for staked and free bandwidth, the contract's
`consume_user_resource_percent` / origin energy limit, and the 1 TRX fee for
activating a new address on TRX transfers.

## Staking and resources

```rust
use tronic::domain::contract::ResourceCode;

client.freeze_balance().amount(trx!(100.0 TRX)).resource(ResourceCode::Energy)
    .build::<AutoSigning>().await?.broadcast(&()).await?;
```

Same pattern for `unfreeze_balance`, `cancel_all_unfreeze`,
`withdraw_unfreeze`, `delegate`, `undelegate`. The builders check
preconditions (enough frozen/delegated TRX, something to withdraw) before
creating the transaction.

## Multisig and external signers

With `ManualSigning` the transaction is built and signed separately, and can
be serialized and passed between participants:

```rust
use tronic::client::pending::{ManualSigning, PendingTransaction};

let mut tx = client.send_trx().to(to).amount(trx!(1.0 TRX))
    .build::<ManualSigning>().await?;
tx.set_permission(2).await?;              // active permission id
let bytes = tx.serialize();

// on the other side
let mut tx = PendingTransaction::try_deserialize(&client, &bytes)?;
tx.sign(&signer_a, &()).await?;
tx.sign(&signer_b, &()).await?;
let txid = tx.broadcast().await?;
```

`sign()` rejects keys that aren't in the permission and duplicate signers;
`broadcast()` checks the accumulated weight against the threshold.
Permissions themselves are changed through `client.account_permissions(addr)`.

To sign with a KMS, HSM or anything else, implement `PrehashSigner`:

```rust
#[async_trait::async_trait]
impl PrehashSigner for MySigner {
    type Ctx = ();
    type Error = MyError;
    async fn sign_recoverable(&self, txid: &Hash32, ctx: &()) -> Result<RecoverableSignature, MyError> { ... }
    fn address(&self) -> Option<TronAddress> { ... }
}
```

For MPC, where you only get a bare ECDSA signature, there is
`sign_tx_unchecked`. It skips the permission checks, so do them yourself.

## Reading blocks

Lower-level calls are on the provider (`use tronic::provider::TronProvider`):

```rust
let p = client.provider();

let head = p.get_now_block().await?.block_header.raw_data.number;
let solid = p.get_solid_block_number().await?;

let block = p.get_block_by_number(n).await?;
for (txid, err) in &block.conversion_errors {
    tracing::warn!(?txid, %err, "transaction not converted");
}

// all receipts of the block in one request
for r in p.get_transaction_info_by_block_num(n).await? {
    match r {
        Ok(info) => { /* info.log, info.internal_transactions, ... */ }
        Err(e) => tracing::warn!(txid = ?e.txid, err = %e.source, "bad receipt"),
    }
}
```

A transaction that can't be converted (unknown contract type, malformed
address) doesn't fail the whole block: unknown types come through as
`ContractType::Unknown { type_id, raw }`, broken ones end up in
`conversion_errors` with their txid.

There is also `client.listener(interval)`, which polls new blocks and fans
them out to subscribers (`TxSubscriber`, `AddressFilter`). It's fine for
watching your own transactions, but it gives no delivery guarantee: the
cursor moves before subscribers process the block, and a slow subscriber can
skip blocks. For indexing or deposit detection, run your own loop on
`get_block_by_number` + `get_transaction_info_by_block_num` and move the
cursor after the block is processed.

## Errors

Everything returns `tronic::error::Error`. The variants worth matching on:

- `NodeRejected { code, message }` — the node refused the request before
  executing it (`DupTransactionError`, `TransactionExpirationError`, bad
  address, …).
- `ContractReverted { message, data }` — a constant call executed and the
  contract reverted.
- `Transport` / `TronProtocol` — network or gRPC level; safe to retry.
- `InsufficientResources(state)` — not enough bandwidth/energy/TRX, with the
  estimate attached.

```rust
use tronic::domain::ResponseCode;
use tronic::error::Error;

match client.provider().broadcast_transaction(tx).await {
    Err(Error::NodeRejected { code: ResponseCode::DupTransactionError, .. }) => { /* already sent */ }
    other => other?,
}
```

## Examples

In [`examples/`](https://github.com/39george/tronic/tree/main/examples):

- [`send_trx.rs`](https://github.com/39george/tronic/blob/main/examples/send_trx.rs) — TRX transfer
- [`usdt_with_multisig.rs`](https://github.com/39george/tronic/blob/main/examples/usdt_with_multisig.rs) — USDT transfer from a multisig account
- [`listener.rs`](https://github.com/39george/tronic/blob/main/examples/listener.rs) — watching USDT transfers

## Development

Generated protobuf code lives in `src/protocol`; `build.rs` regenerates it
only when building the crate itself, not when it's used as a dependency.

Integration tests start a local java-tron node in Docker (see
`tests/helpers/node.rs`), so Docker has to be running:

```sh
cargo test --all-features
```

## Not there yet

- HTTP / JSON-RPC provider (gRPC only for now)
- TRC-10 helpers
- Typed wrappers generated from arbitrary ABIs — for now the call structs are
  written by hand (see `contracts/trc20.rs`)

## License

MIT
