# Solana gRPC Indexer
A simple Solana gRPC indexer written in Rust using the tonic gRPC framework.
It connects to a Solana JSON-RPC endpoint (like Helius) and exposes useful methods over gRPC for querying on-chain data.

⚠️ This is not a standalone full-fledged indexer. Instead, it is a minimal example of how to build a gRPC service around the Solana JSON-RPC API.
<br></br>
<img width="950" height="800" alt="image" src="https://github.com/user-attachments/assets/45608ffd-32bd-4307-b413-fd961819cccd" />





## Features

- Get slot info (`getSlotInfo`)
- Stream slot info (`streamSlotInfo`)
- Get transaction details (`getTransactionInfo`)
- Get account info (`getAccountInfo`)
- Get program accounts (`getProgramAccounts`) ❌(Broken)

## Requirements

- Rust
- Protobuf compiler (`protoc`)
- Solana RPC URL 

## Setup

### 1. Clone the repo

```bash
git clone https://github.com/mrb1nary/rust-gRPC.git
cd rust-gRPC
```

### 2. Enter your Helius RPC URL

```
HELIUS_RPC_URL=https://mainnet.helius-rpc.com/?api-key=your-api-key
```

### 3. Build and Run

```bash
cargo build
cargo run
```


### 4. Directory structure
```bash
├── proto/               # .proto files
├── src/
│   ├── service.rs       # gRPC service implementation
│   ├── methods/         # Request handlers
│   └── main.rs          # Program entrypoint
├── .env
├── Cargo.toml
├── build.rs             # Build file that converts the proto file into rust code
└── README.md
```


### Endpoints
The program exposes 5 endpoints. Use grpcurl or postman to make request

#### 1. GetSlotInfo

```bash
{
    "slot": 265000000  //Input any slot number
}
```


#### 2. GetTransactionInfo

```bash
{
    "signature":"f5S5ctdj2wKPChfa5KX7HHgiWcrvtS7a3pUYidFEqG3UwS84Ncg2Zz2rBtPsReym38xAkGe9UJvRKM3biRGwtrV"  //Enter any txn signature
}
```

#### 3. GetAccountInfo

```bash
{
    "pubkey":"tSg5Ugo5CVuL374natxs6DL8zxXbaBvowqs9Htd2eqd"  //Enter any account's pubkey
}
```

#### 4. GetProgramAccounts

```bash
{
    "program_id": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" //Fetches all the accounts owned by the program ID
}
```

#### 5. StreamSlotInfo

Streams slot height. Directly call the endpoint with empty body
