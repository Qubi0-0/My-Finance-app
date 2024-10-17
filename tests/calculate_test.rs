use finance::calculate_cost_est;
use slint::SharedString;


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