//! TODO: Reinforcement learning bot using Q-learning.
//!
//! States seems too hard to be implemented.
//!
//! ## Algorithm
//!
//! ```shell
//! NewQ(s, a) = Q(s, a) + α[R(s, a) + γ max Q'(s', a') - Q(s, a)]
//! ```
//!
//! - `NewQ(s, a)` New Q value for that state and action
//! - `Q(s, a)` Current Q value
//! - `α` Learning rate
//! - `R(s, a)` Rewards for that action or state
//! - `γ` Discount rate
//! - `max Q'(s', a')` Maximum expected future reward given the new s' and all possible actions at
//!    that new state
//!
//! <https://medium.freecodecamp.org/diving-deeper-into-reinforcement-learning-with-q-learning-c18d0db58efe>
