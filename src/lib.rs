use slint::SharedString;

pub fn calculate_cost_est(cost: f32, time_span: SharedString) -> (f32, f32, f32, f32) {
    let time_span_str: &str = &time_span;
    let (daily, weekly, monthly, yearly) = match time_span_str {
        "Daily" => (cost, cost * 7.0, cost * 30.0, cost * 365.0),
        "Weekly" => (cost / 7.0, cost, cost * 4.2857, cost * 52.1429),
        "Monthly" => (cost / 30.0, cost / 4.2857, cost, cost * 12.0),
        "Yearly" => (cost / 365.0, cost / 52.1429, cost / 12.0, cost),
        _ => (0.0, 0.0, 0.0, 0.0), // Handle unexpected values
    };

    (
        (daily * 100.0).round() / 100.0,
        (weekly * 100.0).round() / 100.0,
        (monthly * 100.0).round() / 100.0,
        (yearly * 100.0).round() / 100.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f32, b: f32, epsilon: f32) {
        assert!((a - b).abs() < epsilon, "left: {}, right: {}", a, b);
    }

    #[test]
    fn test_calculate_cost_est_daily() {
        let cost = 10.0;
        let time_span = SharedString::from("Daily");
        let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);
        assert_approx_eq(daily, 10.0, 0.01);
        assert_approx_eq(weekly, 70.0, 0.01);
        assert_approx_eq(monthly, 300.0, 0.01);
        assert_approx_eq(yearly, 3650.0, 0.01);
    }

    #[test]
    fn test_calculate_cost_est_weekly() {
        let cost = 70.0;
        let time_span = SharedString::from("Weekly");
        let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);
        assert_approx_eq(daily, 10.0, 0.01);
        assert_approx_eq(weekly, 70.0, 0.01);
        assert_approx_eq(monthly, 300.0, 0.01);
        assert_approx_eq(yearly, 3650.0, 0.01);
    }

    #[test]
    fn test_calculate_cost_est_monthly() {
        let cost = 300.0;
        let time_span = SharedString::from("Monthly");
        let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);
        assert_approx_eq(daily, 10.0, 0.01);
        assert_approx_eq(weekly, 70.0, 0.01);
        assert_approx_eq(monthly, 300.0, 0.01);
        assert_approx_eq(yearly, 3600.0, 0.01);
    }

    #[test]
    fn test_calculate_cost_est_yearly() {
        let cost = 3650.0;
        let time_span = SharedString::from("Yearly");
        let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);
        assert_approx_eq(daily, 10.0, 0.01);
        assert_approx_eq(weekly, 70.0, 0.01);
        assert_approx_eq(monthly, 304.17, 0.01);
        assert_approx_eq(yearly, 3650.0, 0.01);
    }
}