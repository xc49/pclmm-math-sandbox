use pclmm_math_sandbox::v2_math;
use pclmm_math_sandbox::v3_math;

fn main() {
    println!("--- AMM Math Sandbox ---\n");

    // V2 Swap
    let r_x = 100.0;
    let r_y = 15000.0;
    let amount_in = 10.0;
    let fee = 0.003;
    println!(
        "Simulation inputs: ReserveX={}, ReserveY={}, InX={}, Fee={}",
        r_x, r_y, amount_in, fee
    );

    let amount_out = v2_math::calculate_swap_output(r_x, r_y, amount_in, fee);
    println!("Swap Results:{} Y", amount_out);

    // Active Capital
    let k = 1e12_f64;
    let p_lower = 0.99;
    let p_upper = 1.01;
    let active = v2_math::calculate_active_capital(k, p_lower, p_upper);
    println!(
        "Active Capital in [{}, {}]: {} X ",
        p_lower, p_upper, active
    );

    // V3 Real Reserves
    let l_depth = 1_000_000.0;
    let current_price = 1.0;
    let p_a = 0.99;
    let p_b = 1.01;
    let (x_real, y_real) = v3_math::calculate_real_reserves(l_depth, current_price, p_a, p_b);
    println!("V3 Real Reserves: x = {}, y = {}", x_real, y_real);
}
