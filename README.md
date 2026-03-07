# SolGuard Multisig

SolGuard is a **programmable multisig treasury built on Solana using the Anchor framework**.  
It enables teams, DAOs, and protocols to securely manage shared funds using **role-based permissions, weighted approvals, and optional timelocks**.

The multisig supports **both SOL transfers and SPL token transfers**, making it suitable for managing on-chain treasuries.

---

# Features

## Role-Based Access Control

Each multisig member can be assigned one or more roles:

- **Proposer** – Can create proposals
- **Approver** – Can approve proposals
- **Executor** – Can execute approved proposals

This allows flexible governance structures where responsibilities are clearly separated.

---

## Weighted Approvals

Approvers can have **custom approval weights**, allowing different voting power among members.

Example:

| Member | Weight |
|------|------|
| Alice | 3 |
| Bob | 2 |
| Charlie | 1 |

If the approval threshold is **4**, then:

- Alice alone cannot approve
- Bob + Charlie can approve
- Alice + Bob easily meets the threshold

---

## Timelocked Proposals

Each proposal can include a **timelock period**.

Even after reaching the required approvals, the proposal **cannot be executed until the timelock expires**.

Benefits:

- Prevents rushed decisions
- Adds a security delay for treasury actions
- Allows time to review potentially malicious proposals

---

# Supported Operations

SolGuard proposals can execute:

### SOL Transfers
Transfer native **SOL** from the multisig vault to any destination address.

### SPL Token Transfers
Transfer **SPL tokens** from the multisig vault token accounts to recipients.

This makes SolGuard usable as a **complete treasury management system**.

---

# Proposal Lifecycle

1. **Create Proposal**

A proposer creates a proposal containing:

- transfer type (SOL or SPL token)
- destination address
- transfer amount
- optional timelock

---

2. **Approve Proposal**

Approvers vote on the proposal.

Each approval contributes its **assigned weight** toward the required threshold.

---

3. **Execute Proposal**

After:

- the **approval threshold is reached**
- the **timelock expires**

an executor can execute the proposal.

Execution performs the transfer from the vault.

---

4. **Close Proposal**

The proposer can close the proposal PDA.

---

# PDA Rent Recovery

SolGuard includes mechanisms to reclaim unused PDA rent.

## Closing a Proposal

- Only the **proposer** can close the proposal
- The **rent from the proposal PDA is transferred to the vault**

---

## Closing the Multisig

Only the **multisig creator** can close the multisig.

When the multisig is closed:

- all PDA rent
- all vault funds
- any remaining assets

are transferred back to the **creator**.

---

# Architecture

SolGuard uses **Program Derived Accounts (PDAs)** to manage program state and authority.

## Core Accounts

| Account | Description |
|------|------|
| Multisig Config | Stores members, roles, weights, and approval threshold |
| Vault | Holds SOL treasury funds |
| Proposal | Stores proposal data |
| Token Vaults | Stores SPL tokens owned by the multisig |

---

# Proposal Data

Each proposal stores:

- proposer
- destination address
- transfer amount
- transfer type (SOL or token)
- token mint (if token transfer)
- timelock period
- approval weight received
- execution status
- multisig reference

---

# Security Design

SolGuard implements multiple security mechanisms:

- PDA-based authorities
- role-based access control
- weighted approvals
- timelock enforcement
- strict account relationship validation
- controlled PDA closures

---

# Built With

- **Solana**
- **Rust**
- **Anchor Framework**

---

# Future Improvements

Potential enhancements:

- CPI calls to external programs
- multi-token treasury dashboards
- proposal simulation before execution
- off-chain notifications
- UI governance dashboard

---

# Example Use Cases

- DAO treasury management
- team shared wallets
- protocol governance
- startup fund management
- investment groups

---

# License

MIT License