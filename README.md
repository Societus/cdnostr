### Secure Relay-Aware Content Distribution Network  
**NIP Draft - Date: [9/4/24]**  
**Title: Secure Relay-Aware Content Distribution Network**

#### Abstract
This proposal outlines a new Nostr Implementation Possibility (NIP) for secure, relay-aware content distribution. It introduces a system where relays can securely store and share encrypted content, acting as blind custodians while also making certain content publicly accessible.

The proposal focuses on three key areas:
1. Secure storage and retrieval of encrypted files, allowing public access while keeping the content encrypted on the relay.
2. Ensuring content remains accessible across multiple relays, even if some are offline.
3. Incentivizing content availability with automated payments to relays, paired with a content health monitoring system to maintain accessibility.

#### Motivation
As Nostr expands beyond simple messaging into file distribution and archiving, a secure mechanism for encrypted content is essential. Current relays aren't equipped for this, especially when redundancy and encryption are needed for secure file sharing and archiving.

Users need a way to store files encrypted at rest but accessible publicly. Relays wouldn’t be able to read the content but could still serve it, lowering liability risks for hosting encrypted files. This proposal addresses several issues:
- Ensuring content is available and secure, even when some relays go offline.
- Supporting public file access with encryption in place, keeping relays "blind" to the content they host.

#### Specification

**Task 1: Secure Storage and Retrieval of Encrypted Files**  
- Relays store encrypted files (blobs) but cannot decrypt them. Content is encrypted before being sent to the relay, using public/private key cryptography.
- Relays act as passive storage, distributing encrypted content upon request.
- A new event type will define encrypted file submission and retrieval. Metadata will include details like file type, encryption method, and verification data (e.g., checksums), with optional features like time-to-live for temporary media.

**Task 2: Relay-Aware Distributed Content Access**  
- Relays will sync content across the network, ensuring redundancy so files remain accessible even if some relays go down.
- A new event type or metadata will track which relays store a particular file, allowing clients to access files from alternative relays.
- Syncing between relays can be done over Tor for extra anonymity, with tags specifying if files are Tor-only.

**Task 3: Blind Custodian for Publicly Accessible Encrypted Files**  
- Relays will serve encrypted files to the public while remaining blind to the content. The decryption key won’t be shared with the relay, but it may be included in the event metadata for authorized users.
- The metadata will include encryption details, public access indicators, and instructions or keys for decryption, ensuring data integrity through checksums.

#### Rationale
This proposal strengthens Nostr’s ability to securely store and retrieve content without relays accessing the data itself. By acting as blind custodians, relays can support both private and public content distribution, adding value in scenarios like distributed archiving, fallback platforms, and secure content caching.

Relays won’t need to know what they’re storing, preserving Nostr’s decentralized ethos while making it easier to handle both public and private content.

#### Security Considerations
- **End-to-End Encryption**: Content is encrypted before it reaches relays. Relays can't decrypt the data, making them passive custodians.
- **Public Decryption**: For public content, decryption keys or instructions are provided in the metadata, ensuring the relay can't read the content.
- **Integrity Checks**: Cryptographic checksums verify that the content hasn't been tampered with.
- **Anonymity**: Communication between relays can be done over Tor for users or relays requiring additional privacy.

#### Backward Compatibility
This proposal is fully backward-compatible. It introduces new event types without affecting the functionality of existing relays or clients. While older systems won’t gain the enhanced capabilities, they will still operate as usual.

#### Implementation Notes
- Define new event types for encrypted storage, relay synchronization, and public file sharing.
- Relays will need extra metadata fields to track content and relay synchronization.
- Clients will handle decryption for public files and check which relays have copies of specific content.
- Synchronization settings should be adjustable to balance network traffic and redundancy.
- Traffic management strategies, such as rate-limiting, will be covered in a separate document (throttle.md).

#### Items for further Consideration
- Expansion of commonly accepted encryption protocols for encrypted storage, probably important for defining a minimum standard for publicly available uploads (thoughts on possible implementation of Signal Protocol key exchange for private uploads in signal.md)
- Integration of blob storage to groups and moderated communities (NIP-29 and NIP-72) in order to define group-operated/owned relays as primary for events in a given group/community
- Additional event type requesting blob archiving of specific content by a participating relay (Wayback Machine via Nostr?)

