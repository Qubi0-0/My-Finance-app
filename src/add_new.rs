// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::finance::*;
use slint::{FilterModel, Model, SortModel};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let state = init();
    let main_window = state.main_window.clone_strong();
    main_window.run().unwrap();
}
fn init() -> State {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    #[rustfmt::skip]
let todo_model = Rc::new(slint::VecModel::<Row>::from(vec![
        Row { name: "Internet".into(), value: 20.0, checked: true, timespan: "Daily".into(), to_delete: false },
        Row { name: "Coffee".into(), value: 120.0, checked: false, timespan: "Daily".into(), to_delete: false },
        Row { name: "Gym Pass".into(), value: 30.0, checked: true, timespan: "Daily".into(), to_delete: false },
    ]));

    let main_window = AppWindow::new().unwrap();

    main_window.on_todo_added({
        let todo_model = todo_model.clone();
        move |text, cost, time_span_val| {
            todo_model.push(Row {
                name: text,
                value: cost,
                checked: true,
                timespan: time_span_val,
                to_delete: false,
            })
        }
    });

    main_window.on_remove_done({
        let todo_model = todo_model.clone();
        move || {
            let mut offset = 0;
            for i in 0..todo_model.row_count() {
                if todo_model.row_data(i - offset).unwrap().to_delete {
                    todo_model.remove(i - offset);
                    offset += 1;
                }
            }
        }
    });

    main_window.on_request_change_cost_estimations({
        let main_window_handle = main_window.as_weak();
        move || {
            let main_window = main_window_handle.unwrap();
            let time_span = main_window.get_selected_timespan();
            let cost = main_window.get_cost();
            let (daily, weekly, monthly, yearly) = calculate_cost_est(cost, time_span);

            main_window.set_day_value(daily);
            main_window.set_week_value(weekly);
            main_window.set_month_value(monthly);
            main_window.set_year_value(yearly);
        }
    });

    main_window.on_apply_sorting_and_filtering({
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();

        move || {
            let window = weak_window.unwrap();
            window.set_todo_model(todo_model.clone().into());

            if window.get_hide_done_items() {
                window.set_todo_model(
                    Rc::new(FilterModel::new(window.get_todo_model(), |e| !e.to_delete)).into(),
                );
            }

            if window.get_is_sort_by_name() {
                window.set_todo_model(
                    Rc::new(SortModel::new(window.get_todo_model(), |lhs, rhs| {
                        lhs.name.to_lowercase().cmp(&rhs.name.to_lowercase())
                    }))
                    .into(),
                );
            }
        }
    });

    main_window.on_sum_expenses({
        let main_window_handle = main_window.as_weak();
        let todo_model_clone = todo_model.clone();
        move || {
            let main_window = main_window_handle.unwrap();
            main_window.set_total_cost(sum_expenses(&todo_model_clone));
        }
    });

    main_window.on_request_fuel_cost_estimations({
        let main_window_handle = main_window.as_weak();
        move || {
            let main_window = main_window_handle.unwrap();
            let fuel_price =  main_window.get__fuel_cost();
            let distance = main_window.get__distance();
            let consumption = main_window.get__consumption();
            main_window.set__total_value(calculate_fuel_cost(fuel_price, distance, consumption));
        }
    });

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.clone().into());
    State {
        main_window,
        todo_model,
    }
}

#[allow(dead_code)]
pub struct State {
    pub main_window: AppWindow,
    pub todo_model: Rc<slint::VecModel<Row>>,
}
