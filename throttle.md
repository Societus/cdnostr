Methods to employ rate-limiting or anti-abuse for relays.

1. Ephemeral Events for Content Availability and Peer Discovery

    Purpose: Relays broadcast ephemeral events containing the state of blob replication, content availability, and relay health.
    Features:
        Relays periodically announce blob availability, peer discovery, and replication status.
        Ephemeral events include tags for blob-id, replication-count, load, threat-level, and ttl (time-to-live).

2. Dynamic Scaling of Event Posting Based on Load and Replication

    Purpose: Relays dynamically adjust event forwarding frequency based on current load and content replication.
    Features:
        Increased event frequency for content nearing expiration or with low replication.
        Reduced event frequency when load is high or replication is sufficient. ## Do not bottleneck existing relay traffic with CDN activity

3. Self-Posting Bootstrap Events

    Purpose: Relays post blob availability events at regular intervals or when blobs are nearing TTL expiration.
    Features:
        Relays announce blob availability and replication needs before blobs expire.
        Other relays or clients can replicate content based on this data.

4. Funding Relay Operations via NIP-57 (Zaps)

    Purpose: Users fund relays hosting content through micropayments via the Bitcoin Lightning Network (Zaps).
    Features:
        Relays expose their cost per GB and available storage.
        Payments (Zaps) are split proportionally based on storage usage and relay hosting costs.
        A Lightning invoice or Zap link is generated to split payments across the relays involved in content storage.

5. Health Events with Combined Funding and Availability Information

    Purpose: Minimize relay activity by combining health, bootstrap, and funding data into a single event.
    Features:
        Events include blob health (availability, load, replication) and relay funding status.
        Tags provide the current funding state and a Zap link for users to contribute proportionally.

6. Client-Hosted Blob Storage

    Purpose: Allow clients to host blobs on behalf of relays, increasing redundancy and reducing relay load.
    Features:
        Clients retrieve blobs from relays and host them locally.
        Clients notify relays with an event specifying the blobs blob-id, host-id, and host-ttl.

7. Content Threat Level Tracking

    Purpose: Dynamically adjust event posting and replication actions based on content nearing TTL expiration or low replication.
    Features:
        Threat levels are calculated based on replication count, relay load, and remaining TTL.
        Higher threat levels lead to more frequent replication attempts and health event posting.

8. Zap Splitting and Proportional Payments

    Purpose: When multiple relays are hosting content, payments for storage are split proportionally.
    Features:
        Zaps are split based on the storage each relay contributes to content hosting.
        Clients receive a combined invoice or Zap link, ensuring payments are distributed fairly across relays.
