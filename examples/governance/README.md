# Governance Contracts

This directory contains contracts implementing decentralized governance functionality, focusing on advanced voting mechanisms and proposal management.

## DAO

The `DAO` contract implements a Decentralized Autonomous Organization with quadratic voting and timelock functionality.

### Architecture

1. **Core Components**
   - Governance token
   - Proposal system
   - Voting mechanism
   - Timelock executor

2. **Voting Power**
   - Quadratic voting
   - Token-based weight
   - Vote delegation
   - Power scaling

3. **Proposal Lifecycle**
   - Creation
   - Voting period
   - Execution delay
   - Implementation

### Key Features

1. **Quadratic Voting**
   ```rust
   // Calculate voting power
   voting_power = sqrt(token_balance)
   
   // Cast vote
   dao.cast_vote(proposal_id, VoteType::For);
   ```

2. **Proposal Management**
   ```rust
   // Create proposal
   dao.propose(
       targets,
       values,
       signatures,
       calldatas,
       description
   );
   
   // Execute proposal
   dao.execute(proposal_id);
   ```

3. **Timelock**
   ```rust
   // Queue transaction
   dao.queue_transaction(
       target,
       value,
       signature,
       data,
       execution_time
   );
   ```

### Proposal States

```rust
enum ProposalState {
    Pending,   // Waiting for voting to start
    Active,    // Voting period active
    Canceled,  // Proposal canceled
    Defeated,  // Did not meet quorum/threshold
    Succeeded, // Passed voting
    Queued,    // In timelock
    Executed,  // Completed
    Expired    // Timelock grace period passed
}
```

### Voting Mechanism

1. **Quadratic Voting**
   - Voting power = √(tokens held)
   - Prevents wealth concentration
   - Encourages participation
   - Fair representation

2. **Vote Types**
   ```rust
   enum VoteType {
       Against = 0,
       For = 1,
       Abstain = 2
   }
   ```

3. **Vote Delegation**
   - Delegate voting power
   - Track delegated power
   - Revoke delegation
   - Delegation history

### Timelock Security

1. **Delay Period**
   - Minimum delay enforcement
   - Maximum delay cap
   - Grace period
   - Emergency procedures

2. **Transaction Queue**
   - Ordered execution
   - Hash-based identification
   - Cancellation mechanism
   - Batch processing

### Configuration Parameters

```rust
struct GovernanceParams {
    voting_delay: Duration,    // Time before voting starts
    voting_period: Duration,   // Duration of voting
    timelock_delay: Duration,  // Execution delay
    proposal_threshold: u256,  // Min tokens to propose
    quorum_votes: u256        // Min participation required
}
```

### Example Usage

1. **Create Proposal**
   ```rust
   let proposal_id = dao.propose(
       vec![target_address],
       vec![0],
       vec!["transfer(address,uint256)"],
       vec![encoded_params],
       "Transfer tokens to treasury"
   );
   ```

2. **Vote on Proposal**
   ```rust
   // Delegate votes
   dao.delegate(delegatee);
   
   // Cast vote
   dao.cast_vote(proposal_id, VoteType::For);
   ```

3. **Execute Proposal**
   ```rust
   // Check state
   assert_eq!(dao.state(proposal_id), ProposalState::Succeeded);
   
   // Execute
   dao.execute(proposal_id);
   ```

### Security Considerations

1. **Voting Security**
   - Vote locking
   - Double-voting prevention
   - Delegation validation
   - Vote weight calculation

2. **Proposal Security**
   - Threshold requirements
   - Timelock enforcement
   - Action validation
   - State transitions

3. **Execution Security**
   - Transaction ordering
   - Failure handling
   - Reentrancy protection
   - Access control

### Testing

The contract includes tests for:
- Proposal lifecycle
- Voting mechanics
- Timelock functionality
- Edge cases
- Security scenarios

Run tests with:
```bash
stremax test examples/governance/
```

### Best Practices

1. **Proposal Creation**
   - Clear descriptions
   - Atomic actions
   - Parameter validation
   - Impact assessment

2. **Voting Process**
   - Adequate voting period
   - Clear vote options
   - Vote transparency
   - Result verification

3. **Execution**
   - Proper delays
   - Failure handling
   - State verification
   - Event emission

4. **Emergency Procedures**
   - Guardian role
   - Emergency actions
   - Pause mechanism
   - Recovery procedures 