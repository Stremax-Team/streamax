# NFT Contracts

This directory contains smart contracts implementing advanced NFT (Non-Fungible Token) functionality with marketplace integration and staking.

## AdvancedNFT

The `AdvancedNFT` contract implements an NFT collection with advanced features like metadata management, rarity scoring, and royalties.

### Key Features

1. **Token Management**
   - ERC721-compatible interface
   - Metadata storage and retrieval
   - Dynamic URI generation
   - Approval system

2. **Minting Mechanics**
   - Public minting
   - Whitelist support
   - Supply management
   - Random attribute generation

3. **Metadata & Rarity**
   - On-chain attributes
   - Rarity scoring
   - Dynamic metadata
   - Trait management

### Usage

```rust
// Deploy collection
let nft = AdvancedNFT::new(
    "Collection Name",
    "SYMBOL",
    "https://api.example.com/metadata/",
    10000,  // max supply
    0.1e18, // mint price
    royalty_recipient,
    500     // 5% royalties
);

// Mint token
let token_id = nft.mint_public();

// Get token metadata
let attributes = nft.get_attributes(token_id);
let rarity = nft.get_rarity_score(token_id);
```

### Security Features

- Reentrancy protection
- Access control
- Input validation
- Supply limits
- Whitelist verification

## NFTMarketplace

The `NFTMarketplace` contract implements a full-featured marketplace for NFT trading.

### Key Features

1. **Listing Types**
   - Fixed price listings
   - Timed auctions
   - Offer system
   - Batch operations

2. **Auction Mechanics**
   - English auctions
   - Minimum bid increments
   - Auto-extending
   - Bid refunds

3. **Payment Handling**
   - Platform fees
   - Royalty distribution
   - Secure transfers
   - Multi-currency support

### Usage

```rust
// Create listing
let listing_id = marketplace.create_listing(
    nft_contract,
    token_id,
    1e18,  // price
    None   // no time limit
);

// Create auction
let auction_id = marketplace.create_auction(
    nft_contract,
    token_id,
    1e18,    // min bid
    0.1e18,  // increment
    Duration::from_days(7)
);

// Make offer
marketplace.make_offer(
    listing_id,
    Duration::from_days(2)
).with_value(1e18);
```

### Fee Structure

1. **Platform Fees**
   - Configurable percentage
   - Basis points calculation
   - Fee recipient management
   - Multi-tier support

2. **Royalties**
   - ERC2981 compatibility
   - Automatic distribution
   - Configurable rates
   - Cross-protocol support

### Security Considerations

1. **Asset Security**
   - Escrow system
   - Transfer validation
   - Balance verification
   - State consistency

2. **Payment Security**
   - Atomic transactions
   - Fee calculation
   - Refund handling
   - Overflow protection

3. **Access Control**
   - Seller verification
   - Buyer validation
   - Admin functions
   - Emergency stops

### Testing

The contracts include tests for:
- Minting operations
- Trading scenarios
- Auction mechanics
- Fee calculations
- Security features

Run tests with:
```bash
stremax test examples/nft/
```

### Best Practices

1. **NFT Implementation**
   - Clear metadata structure
   - Efficient storage
   - Gas optimization
   - Standard compliance

2. **Marketplace Integration**
   - Clean interfaces
   - Event emission
   - Error handling
   - State management

3. **User Experience**
   - Clear pricing
   - Transparent fees
   - Efficient operations
   - Helpful events

4. **Security**
   - Input validation
   - Access control
   - State consistency
   - Emergency procedures

## NFTStaking

The `NFTStaking` contract implements a sophisticated staking system for NFTs with rarity-based rewards and multi-level boost mechanics.

### Key Features

1. **Staking Mechanics**
   - NFT-based staking
   - Time-locked positions
   - Rarity-based rewards
   - Accumulated rewards tracking

2. **Reward System**
   - Dynamic reward rates
   - Rarity multipliers
   - Lock duration boosts
   - Compound rewards

3. **Boost Mechanics**
   - Multi-level boost system
   - Collection-based scoring
   - Progressive thresholds
   - Stackable multipliers

### Usage

```rust
// Initialize staking
let staking = NFTStaking::new(
    nft_contract,
    reward_token,
    1e18,               // 1 token per second
    Duration::from_days(7),  // min stake duration
    5000                // rarity multiplier base
);

// Stake NFT
staking.stake(
    token_id,
    Duration::from_days(30)  // lock duration
);

// Check rewards
let pending = staking.get_pending_rewards(user);

// Claim rewards
staking.claim_rewards();
```

### Reward Calculation

The reward calculation incorporates multiple factors:
```rust
total_reward = base_reward
    * rarity_multiplier
    * lock_duration_boost
    * collection_boost
```

1. **Base Reward**
   - Time-based accrual
   - Global reward rate
   - Pro-rata distribution

2. **Rarity Multiplier**
   - Based on NFT rarity score
   - Configurable multiplier range
   - Dynamic scaling

3. **Lock Duration Boost**
   - Longer locks = higher boost
   - Linear scaling
   - Maximum 1.5x for 1 year

4. **Collection Boost**
   - Based on total staked value
   - Level thresholds:
     - Level 1 (1.2x): 1,000 points
     - Level 2 (1.5x): 5,000 points
     - Level 3 (2.0x): 10,000 points
     - Level 4 (3.0x): 50,000 points

### Security Features

1. **Staking Security**
   - Ownership verification
   - Lock enforcement
   - Safe transfers
   - State consistency

2. **Reward Security**
   - Overflow protection
   - Reward rate limits
   - Update sequencing
   - Balance validation

3. **Boost Security**
   - Score validation
   - Level transitions
   - Multiplier caps
   - State updates

### Testing

The contract includes tests for:
- Staking operations
- Reward calculations
- Boost mechanics
- Time-based scenarios
- Security features

Run tests with:
```bash
stremax test examples/nft/
```

### Integration

1. **NFT Contract**
   - Rarity score access
   - Transfer handling
   - Approval management
   - Event coordination

2. **Reward Token**
   - Distribution control
   - Balance tracking
   - Transfer safety
   - Rate management

3. **User Interface**
   - Stake management
   - Reward tracking
   - Boost visualization
   - Analytics support

### Best Practices

1. **Staking Implementation**
   - Clean state management
   - Efficient calculations
   - Gas optimization
   - Clear validation

2. **Reward Distribution**
   - Fair distribution
   - Accurate tracking
   - Safe transfers
   - Clear documentation

3. **Boost System**
   - Transparent rules
   - Fair progression
   - Clear benefits
   - Easy understanding

4. **Security**
   - Comprehensive validation
   - Safe state changes
   - Clear permissions
   - Emergency handling 