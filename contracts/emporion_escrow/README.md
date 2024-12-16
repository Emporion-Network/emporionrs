# Escrow Contract for Emporion

- Supports handling of CW20 and native assets.
- All transfers **out** of the contract are executed by the **arbiter**.
- An escrow can have one of the following states: **Pending**, **Cancelled**, **Fulfilled**, or **Disputed**.
- Only **Pending** escrows can be modified.
- **Cancelled** escrows return the amount to the payer.
- **Fulfilled** escrows transfer the amount to the beneficiary.
- **Disputed** escrows **split** the amount based on the agreed-upon fractions.
