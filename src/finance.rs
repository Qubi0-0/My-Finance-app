use crate::add_new::Row;
use slint::{Model, SharedString};
use std::rc::Rc;

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

pub fn sum_expenses(expenses_list: &Rc<slint::VecModel<Row>>) -> f32 {
    let sum = expenses_list
        .iter()
        .map(|row| if row.checked { row.value } else { 0.0 })
        .sum();
    sum
}

pub fn calculate_fuel_cost(fuel_price: f32, distance: f32, consumption: f32) -> f32 {
    fuel_price * distance * (consumption / 100.0)
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

    #[test]
    fn test_sum_expenses() {
        #[rustfmt::skip]
        let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
            Row { name: "test1".into(), value: 10.0, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test2".into(), value: 15.3, checked: false, timespan: "Daily".into(), to_delete: false },
            Row { name: "test3".into(), value: 20.0, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test1".into(), value: 69.5, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test2".into(), value: 15.0, checked: false, timespan: "Daily".into(), to_delete: false },
            Row { name: "test3".into(), value: 0.0, checked: true, timespan: "Daily".into(), to_delete: false},
        ]));
        let sum = sum_expenses(&todo_model);
        let expected = 99.5;
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sum_expenses_empty() {
        #[rustfmt::skip]
        let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
            Row { name: "test1".into(), value: 0.0, checked: true, timespan: "Daily".into(), to_delete: false},
        ]));
        let sum = sum_expenses(&todo_model);
        let expected = 0.0;
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sum_expenses_negative_values() {
        #[rustfmt::skip]
        let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
            Row { name: "test1".into(), value: -10.0, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test2".into(), value: -15.3, checked: false, timespan: "Daily".into(), to_delete: false },
            Row { name: "test3".into(), value: -20.0, checked: true, timespan: "Daily".into(), to_delete: false},
        ]));
        let sum = sum_expenses(&todo_model);
        let expected = -30.0;
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sum_expenses_large_values() {
        #[rustfmt::skip]
        let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
            Row { name: "test1".into(), value: 1e10, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test2".into(), value: 2e10, checked: false, timespan: "Daily".into(), to_delete: false },
            Row { name: "test3".into(), value: 3e10, checked: true, timespan: "Daily".into(), to_delete: false},
        ]));
        let sum = sum_expenses(&todo_model);
        let expected = 4e10;
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sum_expenses_nan_values() {
        #[rustfmt::skip]
        let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
            Row { name: "test1".into(), value: f32::NAN, checked: true, timespan: "Daily".into(), to_delete: false},
            Row { name: "test2".into(), value: 15.0, checked: false, timespan: "Daily".into(), to_delete: false },
        ]));
        let sum = sum_expenses(&todo_model);
        assert!(sum.is_nan());
    }

    #[test]
    fn test_calculate_fuel_cost() {
        let consumption: f32 = 4.0;
        let distance: f32 = 100.0;
        let fuel_price: f32 = 4.0;
        let calculated_cost: f32 = calculate_fuel_cost(fuel_price, distance, consumption);
        let expected: f32 = 16.0;
        assert_approx_eq(calculated_cost, expected, 0.1);
    }

    #[test]
    fn test_calculate_fuel_cost_zero_distance() {
        let fuel_price = 5.0;
        let distance = 0.0;
        let consumption = 8.0;
        let expected_cost = 0.0;
        let calculated_cost = calculate_fuel_cost(fuel_price, distance, consumption);
        assert_eq!(calculated_cost, expected_cost);
    }

    #[test]
    fn test_calculate_fuel_cost_zero_consumption() {
        let fuel_price = 5.0;
        let distance = 200.0;
        let consumption = 0.0;
        let expected_cost = 0.0;
        let calculated_cost = calculate_fuel_cost(fuel_price, distance, consumption);
        assert_eq!(calculated_cost, expected_cost);
    }
}
