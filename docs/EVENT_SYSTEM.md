# Notification Event System

## Overview

The notification system delivers healthcare-related alerts (record changes, access events, risk scores, etc.) to users via an on-chain event log. To protect patient privacy and comply with HIPAA, **no plaintext Protected Health Information (PHI) is stored on-chain**.

## Content-Hash Notification Model

### Design

Notification content (title and message body) is never stored in plaintext on the Stellar ledger. Instead:

1. **Sender provides** plaintext `title` and `message` strings at creation time.
2. **Contract computes** a SHA-256 hash of each string: `title_hash` and `message_hash`.
3. **Only the hashes** (`BytesN<32>`) are stored on-chain in the `Notification` record.
4. **Plaintext content** is delivered through the off-chain notification channel (e.g., push notification, email, IPFS) referenced by the `NotificationChannel` preference.

### On-Chain Record

```rust
pub struct Notification {
    pub id: u64,
    pub recipient: Address,
    pub sender: Address,
    pub notif_type: NotificationType,
    pub priority: AlertPriority,
    pub status: NotificationStatus,
    pub title_hash: BytesN<32>,     // SHA-256(title)
    pub message_hash: BytesN<32>,   // SHA-256(message)
    pub reference_id: Option<u64>,
    pub created_at: u64,
    pub read_at: Option<u64>,
    pub expires_at: Option<u64>,
}
```

### Off-Chain Retrieval Flow

1. A client subscribes to on-chain `NotificationCreated` events.
2. The event emits `notif_id`, `recipient`, `sender`, `notif_type`, and `priority`.
3. The client queries the sender's off-chain API or IPFS store using `title_hash` / `message_hash` as content-addressed keys.
4. The off-chain store returns the decrypted plaintext title and message, which are then displayed to the user.

### Privacy Guarantees

- The Stellar ledger is public and permanent. By storing only hashes, no patient-identifiable information is exposed to ledger observers.
- SHA-256 hashes are one-way; the original content cannot be reconstructed from the hash alone.
- The off-chain store can implement additional access controls (e.g., recipient-key encryption via `crypto_registry`) to further restrict who can resolve hashes to plaintext.

### Affected Functions

All three notification creation paths use content hashing:

| Function | Description |
|---|---|
| `create_notification` | Single notification creation |
| `create_bulk_notifications` | Batch creation (up to 20 recipients) |
| `trigger_alert` | Alert-rule-based batch creation |

### Templates

`NotificationTemplate` records store plaintext title and message patterns. These are generic templates (not patient-specific) and do not contain PHI. Templates are validated for length (`MAX_TITLE_LEN`, `MAX_MESSAGE_LEN`) before hashing at notification creation time.
