# PCLMM Math Sandbox (Week 0)
Before we start building the actual smart contract engine, you need to prove you can translate the financial mathematics from Sessions 1 and 2 into working Rust code. 

**[ARCHITECTURAL NOTE]**: In real on-chain smart contracts, floating-point numbers (`f64`) are strictly banned because they cause deterministic rounding errors. However, because this sandbox is an off-chain *simulator*, we are allowing `f64` this week so you can focus strictly on standard Rust syntax and the DeFi formulas. Next week, the training wheels come off.

## First Pull Request

Your objective is to complete the missing mathematical functions in `src/lib.rs` and open a Pull Request (PR) against this repository.

### **Structure:**
- `Cargo.toml`: Project configuration.
- `src/lib.rs`: The core math library where you will implement your logic.
- `src/main.rs`: A simulator to test your implementations manually.

### **The Tasks:**
Open `src/lib.rs`. You will find three modules containing functions with the `todo!()` macro. You need to implement the formulas exactly as we derived them in our sessions.

1. **`v2_math::calculate_swap_output`**: Implement the V2 constant product swap. Apply the fee, calculate the curve shift, and return the final output of Token Y.
   - *Formula Hint*: $dy = \frac{y \cdot dx \cdot (1 - fee)}{x + dx \cdot (1 - fee)}$
2. **`v2_math::calculate_active_capital`**: Prove the "Idle Capital" flaw. Using the provided $k$, $P_{lower}$, and $P_{upper}$, calculate how much capital is actively trading.
3. **`v3_math::calculate_real_reserves`**: Implement the V3 Concentrated Liquidity formulas for $x_{real}$ and $y_{real}$ given liquidity depth $L$, current price $P$, and price bounds $P_a$ and $P_b$.
4. **Write Tests**: Go to the `mod tests` section in `src/lib.rs`. I have implemented `test_v2_swap` for you. You must write the test cases for `test_active_capital` and `test_real_reserves` using the exact numbers from the Session scenarios.

### **Running the Sandbox:**

- **Verify Your Logic**: Run `cargo test`. All tests must pass (turn green).
- **Manual Simulation**: You can use `src/main.rs` to run manual simulations. To run the simulator:
  ```bash
  cargo run
  ```

### **What I Expect From You:**

I'm looking for three things in your PR: **Formula Precision**, **System Reasoning**, and **Verification Rigor**.

#### **1. The Math Implementation**
- **V2 Swap Logic**: Don't just give me the raw output. You must account for the `fee_percentage` correctly. If your output is off by even 0.01, I'll know you didn't apply the fee before the curve shift.
- **Active Capital**: This is where you prove you understood the "Idle Capital" flaw. You need to calculate the Liquidity ($L$) required for the price range we discussed.
- **V3 Real Reserves**: This is the "Aha!" moment for Concentrated Liquidity. You need to translate the $x_{real}$ and $y_{real}$ formulas into code. If you can't handle the relationship between $L$, current price $P$, and your bounds ($P_a, P_b$), your code won't work when we move to the smart contract next week.

#### **2. The Test Suite**
- I've already written `test_v2_swap`. If it stays red, don't even bother opening the PR.
- **Write your own tests**: You must replace the `todo!()` in `test_active_capital` and `test_real_reserves`. Use the **exact numbers** from our Session 1 and 2 . If your tests don't match our session derivations, you haven't mastered the formula yet.

#### **3. Rust Standards**
I'm watching your syntax. I expect to see:
- Clean use of `.sqrt()` and `.powf()`.
- Zero compiler warnings (run `cargo check`).
- A "Green" terminal. If `cargo test` isn't all green, you're not done.

### **The Workflow (How to Submit):**

1. **Fork** this repository to your own GitHub account.
2. **Clone** your forked repo to your local machine.
3. **Create a Branch**: `git checkout -b your-name-math-implementation`
4. **Write Code**: Fill in the `todo!()` blocks in `src/lib.rs`.
5. Ensure `cargo test` passes.
6. Run `cargo fmt` to clean up your code syntax.
7. Push your branch to **your** GitHub fork.
8. Go to the original PCLMM repository and click "Compare & pull request".

### **Review Standards:**
I will be reviewing these PRs personally. 
- If `cargo test` fails, the PR is automatically rejected.
- If your math does not output the exact numbers we derived in the live sessions, the PR is rejected.
- Remember to update your Engineering Journals with whatever Rust syntax confused you the most while building this.
