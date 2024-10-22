// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::finance::calculate_cost_est;
use slint::{FilterModel, Model, SortModel};
use std::rc::Rc;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let state = init();
    let main_window = state.main_window.clone_strong();
    main_window.run().unwrap();
}
fn init() -> State {
    #[rustfmt::skip]
    let todo_model = Rc::new(slint::VecModel::<TodoItem>::from(vec![
        TodoItem { name: "test1".into(), value: 10.0, checked: true, timespan: "Daily".into() },
        TodoItem { name: "test2".into(), value: 15.0, checked: false, timespan: "Daily".into() },
        TodoItem { name: "test3".into(), value: 20.0, checked: true, timespan: "Daily".into() },
    ]));

    let main_window = AppWindow::new().unwrap();

    main_window.on_todo_added({
        let todo_model = todo_model.clone();
        move |text, cost| {
            todo_model.push(TodoItem {
                name: text,
                value: cost,
                checked: false,
                timespan: "Daily".into(),
            })
        }
    });
    main_window.on_remove_done({
        let todo_model = todo_model.clone();
        move || {
            let mut offset = 0;
            for i in 0..todo_model.row_count() {
                if todo_model.row_data(i - offset).unwrap().checked {
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

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    main_window.on_apply_sorting_and_filtering({
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();

        move || {
            let window = weak_window.unwrap();
            window.set_todo_model(todo_model.clone().into());

            if window.get_hide_done_items() {
                window.set_todo_model(
                    Rc::new(FilterModel::new(window.get_todo_model(), |e| !e.checked)).into(),
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

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.clone().into());
    State {
        main_window,
        todo_model,
    }
}

pub struct State {
    pub main_window: AppWindow,
    pub todo_model: Rc<slint::VecModel<TodoItem>>,
}
