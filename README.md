# 🛍️ Mugs Marketplace Contract

A full-featured **Solana NFT Marketplace Smart Contract** with support for **Buy Now**, **Offers**, and **Auction** trading. Built using [Anchor](https://book.anchor-lang.com/) and [Solana](https://solana.com/).

---

## 🚀 Program Deployment

### 📦 Requirements

- Anchor and Rust environment set up
- ~12 SOL in the deployer's wallet
- Configure `Anchor.toml`:

```toml
[programs.devnet]
mugs_marketplace = "<YOUR_PROGRAM_ID>"

[provider]
cluster = "devnet"
wallet = "/home/ubuntu/deploy-keypair.json"
```
---
⚙️ Deployment Steps
# Set up CLI environment
solana config set --url devnet
solana config set --keypair /home/ubuntu/deploy-keypair.json

# Build program
anchor build

# Deploy program
solana program deploy ./target/deploy/mugs_marketplace.so
---
🔁 Changing Program Address
# Remove old keypair
rm ./target/deploy/mugs_marketplace-keypair.json

# Rebuild
anchor build

# Get new address
solana address --keypair ./target/deploy/mugs_marketplace-keypair.json

# Update references:
# - Anchor.toml
# - programs/mugs_marketplace/src/lib.rs (declare_id!)

---
🧪 CLI Command Usage
yarn ts-node cli/command.ts <command>
Error: Provider local is not available on browser.
export BROWSER=

---
🔧 Installation
sudo apt install nodejs yarn
yarn global add ts-node

# Ensure Solana creator wallet exists
# Expected location: /root/.config/solana/creator.json

---
🛠 Initialization Workflow
# Initialize marketplace PDA
yarn ts-node cli/command.ts init

# Set marketplace fee (permyriad)
yarn ts-node cli/command.ts update_fee <sol_fee>

# Add a treasury wallet
yarn ts-node cli/command.ts add_treasury <wallet_address> <rate>

# Initialize user PDA
yarn ts-node cli/command.ts init_user <user_wallet>

---
📚 Command Reference
🛡 Admin
init → Initialize Global PDA

status → View fee, treasury info

update_fee <sol_fee> → Update fee (in permyriad)

add_treasury <wallet> <rate> → Add fee distribution wallet

remove_treasury <wallet> → Remove treasury wallet

👤 User
init_user <wallet> → Initialize user account

user_status <wallet> → Get user balance and volume

🔄 Trading
transfer <mint> <recipient> → Transfer NFT

list <mint> <price_sol> → List NFT for sale

delist <mint> → Cancel listing

purchase <mint> → Buy NFT at list price

💰 Offers
make_offer <mint> <price> → Make offer

cancel_offer <mint> → Cancel offer

accept_offer <mint> <buyer> → Accept offer

🕰️ Auctions
create_auction <mint> <start_price> <min_increase> <duration> <reserve>

place_bid <mint> <price> → Place auction bid

claim_auction <mint> → Claim after auction ends

cancel_auction <mint> → Cancel if no bids

🧾 PDA Info
listed_nft_data <mint> → Get listing data

get_offer_data <mint> <buyer> → Get offer data

get_auction_data <mint> → Get auction info

get_all_listed_nfts → All active listings

get_all_offers_for_nft <mint> → All offers for an NFT

get_all_auctions → All active or unclaimed auctions
---
🌐 Frontend Integration
Use the reusable functions from the /lib directory.

FE developers only need to:

Provide wallet provider + connection

Inject anchor.Program instance (from IDL)

For example usage, refer to cli/scripts/.

Make sure to run:
---
Made with ⚓ Anchor, 🧡 Solana, and 🌍 Open Source.
Let me know if you also want:
- A `CONTRIBUTING.md`
- A prettier version for GitHub Pages or Notion
- Markdown badges (build status, license, etc.) for the top of the file

I can provide them instantly.

