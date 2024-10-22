// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod finance;

use finance::calculate_cost_est;
use std::error::Error;
use std::rc::Rc;
slint::include_modules!();

pub struct State {
    pub ui: AppWindow,
    pub row_model: Rc<slint::VecModel<Row>>,
}

fn init() -> State {

    let row_model = Rc::new(slint::VecModel::<Row>::from(vec![
        Row { name: "test1".into(), value: 10.0, checked: true, timespan: "Daily".into() },
        Row { name: "test2".into(), value: 15.0, checked: false, timespan: "Daily".into() },
        Row { name: "test3".into(), value: 20.0, checked: true, timespan: "Daily".into() },
    ]));

    let ui = AppWindow::new().unwrap();

    ui.on_add_element({
        let row_model = row_model.clone();
        println!("Element added!");
        move |text, cost| row_model.push(Row { name: text, value: cost, checked: false, timespan: "Daily".into() })
    });

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

    State { ui, row_model }
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let state = init();
    let ui = state.ui.clone_strong();

    ui.run()?;

    Ok(())
}