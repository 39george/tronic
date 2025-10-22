# Changelog

## [0.3.2] - 2025-10-22


### Bug Fixes


- Fix to_decimal impl (93ce801)


- Typo (eaf3454)


- Fix macro (8c9bd70)


- Fix mul/div derive (d355fe1)



### Features


- Add serde derive for usdt (c313766)


- Make usdt Hash (5cffe01)


- Implement to_decimal conversion for usdt type (530aecb)


- Add ZERO const (116e89e)


- Add is_wnole method (0fa8a74)


- Add from<T> impl (1ffa3a6)


- Add arithmetic derives (4f2507e)


- Couple helper methods (d4fa3df)


- Implement mock provider (3a05737)


- Add sign_raw_tx_unchecked method for PendingTransaction (4ad743a)


- Add release.toml (32d9f0a)



### Miscellaneous


- Update changelog (ae90e09)


- Do not use protoc when used as dep (2a6d8cd)



### Ci


- Update release.toml (82b42e0)



### Refactor


- Rename to_vec to to_bytes_vec (b81329d)


- Format (21dc10d)


- Use FromStr instead of TryFrom<&str> for hashes (fd60768)


- More verbose txid from digest (fa5764e)


- Use &[u8] for try_deserialize PendingTransaction (39ea54a)


- Rename sign_raw_tx_unchecked to sign_tx_unchecked (2a2099d)


## [v0.3.1] - 2025-08-06


### Bug Fixes


- Fix build.rs (264373f)


- For cargo publish (d5ecfac)


- Refresh txid on transaction signing (manual) (08e6e33)


- Regression (399e6b9)



### Features


- Sign method now returns RecoverableSignature (87242e6)



### Miscellaneous


- Update gitignore (6ce7bd5)


- Format (cf50e7c)


- Bump version (42790b2)



### Refactor


- Add assert for TronAddress test (2a1e44b)



### Test


- Implement new integration test (d3b77ee)


## [v0.3.0] - 2025-08-06


### Bug Fixes


- Fix estimation bug (f5a5aaa)


- Fix timestamp generation for new tx (8f9cba7)



### Features


- Add serde derives for main domain types (28d5ac1)


- Pass txid to event handler for TxSubscriber (28c4574)


- Make block poll interval customizable (39af5fe)


- Use TxCode type instead of i32 for TransactionResult (d9e24a9)


- Implement freeze_balance_builder, delegate&freeze types (2cd38d7)


- Implement unfreeze_balance_v2, cancel_all_unfreeze (8d0ae41)


- Implement delegation (4013a67)


- Implement undelegate (b372354)


- Implement withdraw expire unfreeze (4fd2ae0)


- Add https scheme support for grpc provider (c09e27b)


- Open permission fields to read (c01399c)


- Implement contract upload (b4e6acf)


- Implement broadcast confirmation receipt (640354a)


- Implement contract read call (b2e8839)


- Better error report for perissions (24ef31b)



### Miscellaneous


- Update CHANGELOG (b1a4f2e)


- Bump version in Crago.lock (2ebdb79)


- Add couple helper methods (6d61734)


- Disable Default tracing warn logging (009ca58)


- Bump tonic & prost versions (cfbb837)


- Bump version (7bfdb84)



### Refactor


- Better error msgs (ca004d7)


- Make permission getters pub (8d2c6fe)


- Make some permission fileds copy (getters) (b75245b)


- Extract trc20 calls to separate trait (28b1beb)


- Use method instead of property for provider (35fc28e)



### Test


- Implement integration tests module (2 new freeze tests) (0c37608)


- Implement tests for trc20 contract upload (8c5b183)


## [v0.2.1] - 2025-07-27


### Bug Fixes


- Fix example (e2a8b38)


- Now expiration works (f5d012f)


- Fix bandwidth estimation logic (813365a)



### Documentation


- Update changelog for 0.1.1 (cd8f97d)



### Features


- Implement bandwidth estimation (604ae87)


- Return token type (44884c7)


- Implement fee estimation && checks (d553667)


- Add balance check to trc20 transfer builder (2f0cada)


- Add trx transfer balance check (44d89f0)


- Make api faster with parallel calls with try_join macro call (b825c14)


- Add send_trx example (71c6be4)


- Implement usdt_with_multisig example, better fee estimation (3ada4a3)



### Miscellaneous


- Remove unused comment (fc294de)


- Downgrade tonic&prost back (b2e35c4)


- Update Cargo.lock (937840a)


- Clean (5439e99)


- Update README (2f2fd43)


- Add CONTRIBUTING.md (f9f5950)


- Update README (cb8e0d9)


- Update README (3919cac)


- Update README (b197347)


- Add doc comments for listener example (074b774)


- Update README (5c3b219)


- Bump version (43cb1ae)



### Refactor


- Move energy/bandwidth prices methods to client (39a58b1)


## [v0.1.1] - 2025-07-25


### Bug Fixes


- Explicit lifetimes (af36936)


- Add GrpcProvider: Clone (3f3c37c)


- Fix todos (ab486ef)



### Features


- Add From<SigningKey> conversion (1c081ab)


- New method, conversions (216407a)


- Implement trigger_constant_contract (42d4536)


- Implement getting trx, trc20 balances (fe77f82)


- Make Client: Clone (f934891)


- Add block listener support (e103d10)


- Add Debug impls for domain types (492d6ae)


- Better debug printing (0d67977)


- Implement listener & subscriber pair to listen for blocks (5bf9223)


- Begin proofofconcept impl (b46d7bf)


- Restrict closure handler to Clone (69d08a1)


- Start implementation (1aa19ee)


- Implement more conversions (b1356ce)


- More conversions (069717f)


- Finish (6f3e520)


- Improve filtering (b325676)


- Wrap into Arc (929edf4)


- Implement AddressFilter (d0341ed)


- Implement contract methods (1e7ea70)


- Implement better design for trc20 (2a5f323)


- Better type model (2f17172)


- Improve type model (60327b6)


- Implement fixed_string macro, update acc permission call (2b4c983)


- Add permission interface (a94248a)


- Add builder for Permissions (f413dfa)


- Implement get transaction info by txid (2ea15bf)


- Add derives (3fe73ec)



### Miscellaneous


- Add crate metadata, add license files (773d862)


- Update doc comments (d9f8ff2)


- Add listener example (91d2f94)


- Bump version (0d0329d)


- Use stable channel (07c00dc)


- Bump version to 0.1.1 (4203334)



### Refactor


- Remove redunant conversion (9153470)


- Use BlockExtention as event (120cc11)


- Use macro for struct gen (c636de1)


- Extract trait into provider module (58ee7b0)


