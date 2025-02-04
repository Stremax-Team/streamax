# DeFi Contracts

This directory contains Decentralized Finance (DeFi) smart contracts demonstrating various financial primitives and mechanisms.

## LiquidityPool

The `LiquidityPool` contract implements an Automated Market Maker (AMM) with flash loan capabilities.

### Key Features

1. **Constant Product AMM**
   - Uses x * y = k formula for price determination
   - Maintains price stability through arbitrage
   - Supports efficient price discovery

2. **Liquidity Management**
   - Add liquidity with balanced token pairs
   - Remove liquidity maintaining pool ratio
   - Fair share calculation based on contribution
   - Anti-slippage protection

3. **Flash Loans**
   - Borrow tokens without collateral
   - Must repay within same transaction
   - Includes fee mechanism
   - Callback pattern for loan usage

### Usage

```rust
// Initialize pool
let pool = LiquidityPool::new(token_a, token_b);

// Add liquidity
pool.add_liquidity(1000e18, 1000e18);

// Perform swap
pool.swap(token_a, 100e18);

// Use flash loan
pool.flash_loan(token_a, 1000e18, callback_data);
```

### Security Features

- Reentrancy protection on all state-modifying functions
- Comprehensive input validation
- Balance checks before transfers
- Fee collection before loan completion

## StakingRewards

The `StakingRewards` contract implements an advanced staking system with boosting and vesting capabilities.

### Key Features

1. **Flexible Staking**
   - Stake and unstake at any time
   - Dynamic reward calculation
   - Real-time reward tracking
   - Multiple reward distribution models

2. **Boost System**
   - Lock-time based multipliers
   - 1x-3x boost range
   - Customizable lock periods
   - Boost decay mechanism

3. **Vesting Schedules**
   - Linear and exponential vesting
   - Customizable cliff periods
   - Time-based unlocking
   - Vesting schedule management

### Usage

```rust
// Initialize staking
let staking = StakingRewards::new(
    staking_token,
    reward_token,
    reward_rate,
    reward_duration
);

// Stake tokens
staking.stake(1000e18);

// Set boost with 1 year lock
staking.set_boost(Duration::from_days(365));

// Claim rewards
staking.get_reward();
```

### Reward Calculation

The reward calculation uses a "rewards per token" approach:
1. Track global rewards per token
2. Store user's last rewards per token
3. Calculate difference for user's rewards
4. Apply boost multiplier

Formula:
```
user_reward = staked_amount * (current_reward_per_token - user_last_reward_per_token)
boosted_reward = user_reward * boost_multiplier
```

### Security Considerations

1. **Reward Gaming Prevention**
   - Minimum lock periods
   - Maximum boost caps
   - Update rewards before state changes
   - Sandwich attack protection

2. **Vesting Security**
   - Immutable schedules
   - Time-based validation
   - Cliff period enforcement
   - Proper balance tracking

3. **State Management**
   - Atomic operations
   - Proper order of operations
   - Event emission
   - Balance reconciliation

## Testing

Both contracts include comprehensive test suites covering:
- Basic functionality
- Edge cases
- Time-dependent scenarios
- Security considerations

Run tests with:
```bash
stremax test examples/defi/
``` 