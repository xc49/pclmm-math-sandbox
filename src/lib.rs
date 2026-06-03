//! # AMM Math Sandbox
//!
//! WARNING: While actual Solana smart contracts strictly ban floating-point numbers (f64)
//! to ensure determinism and avoid precision issues, this specific off-chain simulator
//! uses f64 to easily translate standard algebraic formulas for learning purposes.
//! //! I intentionally took examples out of the sessions , so please keep that in mind.

#[allow(unused_variables)]
pub mod v2_math {
    /// Calculates the output amount for a swap given reserves and input.
    /// Formula: dy = (y * dx * (1 - fee)) / (x + dx * (1 - fee))
    pub fn calculate_swap_output(
        reserve_x: f64,
        reserve_y: f64,
        input_x: f64,
        fee_percentage: f64,
    ) -> f64 {
        let fee_multiplier = 1.0 - fee_percentage;
        let numerator = reserve_y * input_x * fee_multiplier;
        let denominator = reserve_x + input_x * fee_multiplier;
        numerator / denominator
    }

    /// Calculates the active capital (L) for a given k and price range.
    pub fn calculate_active_capital(k: f64, p_lower: f64, p_upper: f64) -> f64 {
        let x1: f64 = (k / p_upper).sqrt();
        let x2: f64 = (k / p_lower).sqrt();
        x2 - x1
    }
}

#[allow(unused_variables)]
pub mod v3_math {
    /// Calculates real reserves for a given liquidity depth and price range.
    pub fn calculate_real_reserves(
        l_depth: f64,
        current_price: f64,
        p_a: f64,
        p_b: f64,
    ) -> (f64, f64) {
        let x_reserve: f64 = l_depth * (1.0_f64 / current_price.sqrt() - 1.0_f64 / p_b.sqrt());
        let y_reserve: f64 = l_depth * (current_price.sqrt() - p_a.sqrt());
        (x_reserve, y_reserve)
    }
}

#[cfg(test)]
mod tests {
    use super::v2_math::*;
    use super::v3_math::*;

    #[test]
    fn test_v2_swap() {
        let reserve_x = 100.0;
        let reserve_y = 15000.0;
        let input_x = 10.0;
        let fee_percentage = 0.003;

        let output: f64 = calculate_swap_output(reserve_x, reserve_y, input_x, fee_percentage);

        // Assert that the output is approximately 1359.92
        assert!(
            (output - 1359.92).abs() < 0.01,
            "Expected ~1359.92, got {}",
            output
        );
    }

    #[test]
    fn test_active_capital() {
        let k = 1e12_f64;
        let p_lower = 0.99;
        let p_upper = 1.01;
        let active_capital = calculate_active_capital(k, p_lower, p_upper);
        // Assert that the active capital is approximately 10000.62
        assert!(
            (active_capital - 10_000.63).abs() < 1.0,
            "Expected ~10000.63, got {}",
            active_capital
        );
    }

    #[test]
    fn test_real_reserves() {
        let l_depth = 1000000.0;
        let current_price = 1.0;
        let p_a = 0.99;
        let p_b = 1.01;
        let (x_reserve, y_reserve) = calculate_real_reserves(l_depth, current_price, p_a, p_b);
        // Assert that the real reserves are approximately (1000.0, 1000.0)
        assert!(
            (x_reserve - 4962.81).abs() < 0.01,
            "Expected ~4962.8, got {}",
            x_reserve
        );
        assert!(
            (y_reserve - 5012.56).abs() < 0.01,
            "Expected ~5012.56, got {}",
            y_reserve
        );
    }
}
