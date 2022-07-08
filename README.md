# didcomm-protocols

[![Build Status](https://github.com/chriamue/didcomm-protocols/actions/workflows/coverage.yml/badge.svg)](https://github.com/chriamue/didcomm-protocols/actions)
[![codecov](https://codecov.io/gh/chriamue/didcomm-protocols/branch/main/graph/badge.svg?token=QEH2EW6LX4)](https://codecov.io/gh/chriamue/didcomm-protocols)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Some protocols constructed atop DIDComm Messaging

## quickstart

```sh
cargo build
cargo test
```

## wasm

```sh
wasm-pack test --node
```

## Protocols

| Protocol                                                                                                  | Not started |     In Development     | In Review |        Done        | Notes                    |
| :-------------------------------------------------------------------------------------------------------- | :---------: | :--------------------: | :-------: | :----------------: | :----------------------- |
| [basic message](https://didcomm.org/basicmessage/2.0/)                                                    |             | :large_orange_diamond: |           |                    |                          |
| [did exchange](https://github.com/hyperledger/aries-rfcs/blob/main/features/0023-did-exchange)            |             | :large_orange_diamond: |           |                    |                          |
| [issue credential](https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2) |             | :large_orange_diamond: |           |                    |                          |
| [present proof](https://github.com/hyperledger/aries-rfcs/blob/main/features/0454-present-proof-v2)       |             | :large_orange_diamond: |           |                    |                          |
| [report problem](https://github.com/hyperledger/aries-rfcs/blob/main/features/0035-report-problem)        |             | :large_orange_diamond: |           |                    |                          |
| [trust ping](https://identity.foundation/didcomm-messaging/spec/#trust-ping-protocol-20)                  |             |                        |           | :heavy_check_mark: | Finished implementation. |