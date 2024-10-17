// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.on_request_change_cost_estimations({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let time_span = ui.get_selected_timespan();
            let cost = ui.get_cost();
            let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);

            ui.set_day_value(daily);
            ui.set_week_value(weekly);
            ui.set_month_value(monthly);
            ui.set_year_value(yearly);
        }
    });

    ui.run()?;

    Ok(())
}

fn calculate_cost_est(cost: f32, time_span: SharedString) -> (f32, f32, f32, f32) {
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

