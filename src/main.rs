// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use finance::*;
use std::error::Error;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    

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
