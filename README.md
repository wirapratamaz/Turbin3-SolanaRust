# 🚀 Turbin3-SolanaRust

![Solana](https://img.shields.io/badge/Solana-black?style=for-the-badge&logo=solana&logoColor=14F195)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## 📝 Overview

This project demonstrates interacting with Solana using Rust. It implements wallet operations, SOL transfers, and program interactions on Solana's devnet. The project was completed as part of the Turbin3 pre-requisites program.

## 🛠️ Setup & Installation

### Prerequisites

- Rust and Cargo installed
- Solana CLI (optional)

### Installation

1. Clone this repository
   ```bash
   git clone https://github.com/wirapratamaz/Turbin3-SolanaRust.git
   cd Turbin3-SolanaRust
   ```

2. Install dependencies
   ```bash
   cargo build
   ```

## ✨ Features

This project demonstrates how to:

- ✅ Generate a new Solana keypair
- ✅ Save and load wallets
- ✅ Convert between wallet formats (base58 ↔ byte array)
- ✅ Request airdrops from Solana devnet
- ✅ Transfer SOL between accounts
- ✅ Empty a wallet
- ✅ Interact with Solana programs
- ✅ Use Program Derived Addresses (PDAs)
- ✅ Submit on-chain transactions with custom data

## 📊 Completed Tasks

| Task | Description | Transaction | Status |
|------|-------------|-------------|--------|
| 1. Create Wallet | Generated a new Solana keypair | [4jyFUqKD7vJCHUy7YLV7qMes3uuxRSNmfJ84XJNSdL1z](https://explorer.solana.com/address/4jyFUqKD7vJCHUy7YLV7qMes3uuxRSNmfJ84XJNSdL1z?cluster=devnet) | ✅ |
| 2. Airdrop SOL | Claimed devnet SOL tokens | [4EbyDvdAefjDa7ALhfMriQzQ9mQS349aDdzAkJiCdk5NnL5ZxLNGDsNUtiKv7nwbbJkdzxWAR7qvmKPnuSb3vi6y](https://explorer.solana.com/tx/4EbyDvdAefjDa7ALhfMriQzQ9mQS349aDdzAkJiCdk5NnL5ZxLNGDsNUtiKv7nwbbJkdzxWAR7qvmKPnuSb3vi6y?cluster=devnet) | ✅ |
| 3. Transfer SOL | Sent SOL to Turbin3 wallet | [WbU364CRjqdNGmN21y8RxUjDB8SDj6ZmRE5aRahHZ2JXM9PuMbJiViaFKRhs3EV1PHohydijhtEf4RsNFQUENzB](https://explorer.solana.com/tx/WbU364CRjqdNGmN21y8RxUjDB8SDj6ZmRE5aRahHZ2JXM9PuMbJiViaFKRhs3EV1PHohydijhtEf4RsNFQUENzB/?cluster=devnet) | ✅ |
| 4. Empty Wallet | Transferred all remaining SOL | [3VitV9JTji6kKihES3GGooit3Lt2B182SUvea3YiEW6JPQKqqL5sRftqT4ZZ5azyWkjpjKgqs33kfZVzpxaBAN1p](https://explorer.solana.com/tx/3VitV9JTji6kKihES3GGooit3Lt2B182SUvea3YiEW6JPQKqqL5sRftqT4ZZ5azyWkjpjKgqs33kfZVzpxaBAN1p?cluster=devnet) | ✅ |
| 5. Enrollment | Completed Turbin3 enrollment | [YMCkN3MSJY4QP2rCnsCeN8k3PfrtfffEu7Z8GEsVxyTeiTSmVB5oWBJ5GB9C1VnA3vUsfziWFqBY9pmkavVrFWs](https://explorer.solana.com/tx/YMCkN3MSJY4QP2rCnsCeN8k3PfrtfffEu7Z8GEsVxyTeiTSmVB5oWBJ5GB9C1VnA3vUsfziWFqBY9pmkavVrFWs/?cluster=devnet) | ✅ |

## 💻 Usage

### Generate a New Keypair

```bash
cargo test keygen -- --nocapture
```

Example output:
```
You've generated a new Solana wallet: GSxitshqZfyVt6wPakV4pkqjjLiuBehogdt98Af7fmss

To save your wallet, copy and paste the following into a JSON file:
[72,101,108,112,32,109,101,33,32,73,32,97,109,32,116,114,97,112,112,101,100,32,105,110,32,97,32,119,97,108,108,101,116,32,102,97,99,116,111,114,121]
```

### Request an Airdrop

```bash
cargo test airdop -- --nocapture
```

Example output:
```
Success! Check out your TX here:
https://explorer.solana.com/tx/4EbyDvdAefjDa7ALhfMriQzQ9mQS349aDdzAkJiCdk5NnL5ZxLNGDsNUtiKv7nwbbJkdzxWAR7qvmKPnuSb3vi6y?cluster=devnet
```

### Transfer SOL

```bash
cargo test transfer_sol -- --nocapture
```

Example output:
```
Signature verified
Success! Check out your TX here: 
https://explorer.solana.com/tx/WbU364CRjqdNGmN21y8RxUjDB8SDj6ZmRE5aRahHZ2JXM9PuMbJiViaFKRhs3EV1PHohydijhtEf4RsNFQUENzB/?cluster=devnet
```

### Empty Wallet

```bash
cargo test empty_wallet -- --nocapture
```

Example output:
```
Current balance: 1998995000 lamports
Transaction fee: 5000 lamports
Success! Check out your TX here: 
https://explorer.solana.com/tx/3VitV9JTji6kKihES3GGooit3Lt2B182SUvea3YiEW6JPQKqqL5sRftqT4ZZ5azyWkjpjKgqs33kfZVzpxaBAN1p/?cluster=devnet
New balance: 0 lamports
```

### Turbin3 Enrollment

```bash
cargo test complete_enrollment -- --nocapture
```

Example output:
```
=====================================================
Submitting Turbin3 Enrollment
=====================================================
Signer Public Key: 4jyFUqKD7vJCHUy7YLV7qMes3uuxRSNmfJ84XJNSdL1z
Current balance: 2000000000 lamports

Creating PDA with inputs:
  - Seed: prereq
  - Public Key: 4jyFUqKD7vJCHUy7YLV7qMes3uuxRSNmfJ84XJNSdL1z
PDA derived: HNaVq4jH1nHjDt4i3d82cXSfCgNdraTTpj5xp2vEkGXK

Program Details:
  - Program ID: ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa
  - Github Username: wirapratamaz

Getting recent blockhash...
Building and sending transaction...

Success! 🎉
Your enrollment is now recorded on-chain!
Transaction: https://explorer.solana.com/tx/YMCkN3MSJY4QP2rCnsCeN8k3PfrtfffEu7Z8GEsVxyTeiTSmVB5oWBJ5GB9C1VnA3vUsfziWFqBY9pmkavVrFWs/?cluster=devnet
=====================================================
```

## 🔐 Security Notes

- `.gitignore` is configured to exclude wallet files to prevent committing private keys
- Always create a new wallet for different purposes/projects
- Keep your private keys safe and never share them

## 🌐 Resources

- [Solana Documentation](https://docs.solana.com/)
- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Solana Explorer](https://explorer.solana.com/?cluster=devnet)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

<p align="center">
  <sub>Developed by <a href="https://github.com/wirapratamaz">wirapratamaz</a> for Turbin3 Solana Rust Program</sub>
</p> 