# 🦀 tronic

[![Crates.io](https://img.shields.io/crates/v/tronic)](https://crates.io/crates/tronic)
[![docs](https://docs.rs/tronic/badge.svg)](https://docs.rs/tronic/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/39george/tronic/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/tronic)](https://crates.io/crates/tronic)

> Un cliente de Rust modular, seguro en tipos y orientado a async para la blockchain de Tron — inspirado en Alloy y construido para la interacción con contratos inteligentes en el mundo real.

---

## Características

### Infraestructura Core
- **Llamadas a Contratos Inteligentes Tipadas** — Impulsadas por macros de `alloy-sol-types` para seguridad en tiempo de compilación.
- **Soporte de Protocolo Dual** — Proveedores gRPC (vía `tonic`) y JSON-RPC (WIP).
- **Firmadores Enchufables (Pluggable)** — Firmadores locales o backends de firma async personalizados.
- **Async-First** — Construido sobre Tokio para una interacción de alto rendimiento con la blockchain.

### Gestión de Cuentas
- **Soporte Multi-firma** — Flujo completo para firmas de umbral (threshold signatures).
- **Gestión de Permisos** — Modifica los permisos de la cuenta programáticamente.
- **Delegación de Recursos** — Manejo de congelación/descongelación de ancho de banda y energía (WIP).

### Interacción con Contratos Inteligentes
- **TRC-20** — Transferencias de tokens seguras en tipos con builders al estilo `alloy`.
- **Codegen de ABI de Contratos** — Genera tipos desde ABIs de Solidity (actualmente requiere implementar un wrapper manualmente).
- **Filtrado de Eventos** — Suscripción de eventos rica y consultas históricas.
- **Estimación de Transacciones** — Cálculo preciso de energía y ancho de banda con modos de respaldo (fallback).

### Características Avanzadas de Transacciones
- **Agrupación de Transacciones (Batching)** — Agrupa múltiples operaciones atómicamente (WIP).
- **Manejo de Plazos (Deadline)** — Gestión automática/manual de la expiración de transacciones.

---


## Inicio Rápido

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

## Aprender con Ejemplos

Explora escenarios de uso prácticos en nuestro [directorio de ejemplos](https://github.com/39george/tronic/tree/main/examples):

- [`Multisig`](https://github.com/39george/tronic/blob/main/examples/usdt_with_multisig.rs) - Transferencia de USDT multi-firma.
- [`Event listener`](https://github.com/39george/tronic/blob/main/examples/listener.rs) - Monitoreo de transferencias de USDT en tiempo real.
- [`Trx transfer`](https://github.com/39george/tronic/blob/main/examples/send_trx.rs) - Ejemplo simple de transferencia de TRX.

## TODO

- [ ] Implementar batching
- [ ] Pruebas unitarias y de integración
- [ ] Más ejemplos
- [ ] Preparar documentación
