# Master list of ideas for existing projects that can be formed into a nostr CDN

1) new NIP for assigning relay awareness for IPFS gateways
   - Would need to have prep steps to make IPFS content custodially blind
   - relays would need a function to monitor and report IPFS gateway content liveness, ideally with proposed nip-66 (nostr-protocol pull #230)
   - Possibly allow relays to operate their own IPFS node, and use events to do relay discovery to dynamically scale an IPFS cluster
   - re: dynamic cluster scaling - would be great to use nostr auth mechanism to grant access to IPFS nodes run by relays, would make a paid relay and paid storage model, even opening to something like definable costs per node with funding via zap splits.

2) Create S3 integration for NIP-96 file servers
   - Likely to be stupid expensive
   - Would only work for paid relays because of above reason
   - Fast
   - Client-side encryption could be used for self-signed content, giving a method for S3 to be less likely to censor uploads
   - Plenty of self hostable S3 platforms
  
3) new NIP that controls a self-pruning blockchain
   - Chia or Filecoin model, with increased emphasis on content expiration based on nostr events
   - I don't really like this, but seems like the easiest possible implementation
   - Tribler.org has a pseudo blockchain called TrustChain used for rate limiting freeleechers, might be perfect for this.
