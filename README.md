# V-Zero Protocol: Frontend Dashboard

The decentralized user interface for the **V-Zero Protocol**, providing a confidential payroll workspace, contributor dashboard, and compliance audit client built with Next.js, React, and Tailwind CSS.

---

## 🏗️ Architecture Integration

The frontend serves as the secure client runtime where privacy is enforced before interacting with the network:

+------------------+         Deterministic Signature         +-------------------------+
|  Freighter UI    | --------------------------------------> | Private Viewing Key Ivsk|
|  (User Wallet)   |                                         | (Strictly Client-Side)  |
+------------------+                                         +------------+------------+
|                                                                |
| Signs Encrypted Payload                                        | Decrypts Logs
v                                                                v
+--------+---------+         Type-Safe Actions                       +----+--------------------+
| Frontend Core UI | --------------------------------------> | Auto-Generated Bindings |
| (Next.js App)    |                                         | (vzero-client SDK)      |
+------------------+                                         +------------+------------+
|
| Dispatches WASM Transaction
v
+------------+------------+
|  Stellar Network Ledger |
+-------------------------+


* **Client-Side Cryptography:** Private viewing keys ($Ivsk$) are derived locally via deterministic wallet signatures. Plaintext records and rosters never touch the blockchain.
* **Type-Safe Gateway:** All state modification parameters are strictly checked using an auto-scaffolded `vzero-client` package built directly from contract bytecode.

---

## 🚀 Key Features

* **Freighter Wallet Integration:** Secure connection and cryptographic authentication via standard Stellar browser extension wallets.
* **Deterministic Privacy Layer:** Local derivation of private viewing keys ($Ivsk$) via secure wallet signatures, ensuring financial logs stay strictly client-side.
* **Type-Safe Contract Interactivity:** Direct integration with live Soroban smart contracts using auto-scaffolded TypeScript bindings.
* **Responsive Architecture:** Beautiful, modern workspace layout built with optimized flexbox compositions.

---

## 🛠️ Project Setup & Installation

### 1. Install UI System Dependencies
Clone the repository, verify that any duplicate lockfiles are cleared, and install the required workspace modules:

```bash
# Clear alternative lockfiles if present to prevent package conflicts
rm -f pnpm-lock.yaml

# Install dependencies
npm install
2. Compile On-Chain Client Bindings
Ensure the internal type-safe smart contract client sub-package is fully initialized and built:

Bash
# Change to the client bindings directory
cd src/lib/vzero-client

# Install binding packages and compile modules
npm install
npm run build
💻 Local Development
Navigate back out to the primary project root directory and spin up the hot-reloading local Next.js development engine:

Bash
# Step back out to frontend project root
cd ../../../

# Run development server
npm run dev
Open http://localhost:3000 inside your browser to view the application interface, connect your testnet wallet, and interact with the protocol!

⚙️ Build for Production
To assemble optimized production bundles ready for public deployment:

Bash
npm run build
