cargo_component_bindings::generate!();

use bindings::*;
use conversions::{
    from_wit_data, from_wit_moovable, from_wit_moovables, into_wit_moovable, into_wit_trend,
};
use crate::bindings::exports::mooving::moovables::api::{
    Guest, Data as WitData, Moovable as WitMoovable, MoovableResult as WitMoovableResult,
    Moovables as WitMoovables, TrendResult as WitTrendResult,
};
// use exports::mooving::moovables::api::{
//     Api, Data as WitData, Moovable as WitMoovable, MoovableResult as WitMoovableResult,
//     Moovables as WitMoovables, TrendResult as WitTrendResult,
// };
use mooving::{MoovingState, NEW_MOOVING_STATE};

mod conversions;
struct Component;

struct State(MoovingState);

static mut STATE: State = State(NEW_MOOVING_STATE);

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    unsafe { f(&mut STATE) }
}

fn prepend<T>(items: Vec<T>, item: T) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = vec![item];
    tmp.extend(items);
    tmp
}

impl Guest for Component {
    fn set_goal(goal: u32) {
        with_state(|State(state)| {
            state.goal = goal;
        })
    }

    fn get_goal() -> u32 {
        with_state(|State(state)| state.goal)
    }

    fn add_moovable(name: String, moovable_type: String) -> WitMoovableResult {
        with_state(|State(state)| {
            if state.moovables.iter().any(|m| m.name == name) {
                WitMoovableResult::Error("Moovable already exists".to_string())
            } else {
                let moovable = WitMoovable {
                    name,
                    moovable_type,
                    current: false,
                    data: Vec::new(),
                };
                state.moovables.push(from_wit_moovable(moovable.clone()));
                WitMoovableResult::Ok(moovable)
            }
        })
    }

    fn get_moovables() -> Vec<WitMoovable> {
        with_state(|State(state)| {
            state
                .moovables
                .clone()
                .into_iter()
                .map(into_wit_moovable)
                .collect::<Vec<WitMoovable>>()
        })
    }

    fn get_moovable(name: String) -> WitMoovableResult {
        with_state(|State(state)| {
            if let Some(bike) = state
                .moovables
                .iter()
                .find(|moovable| moovable.name == name)
            {
                WitMoovableResult::Ok(into_wit_moovable(bike.clone()))
            } else {
                WitMoovableResult::Error("Bike not found".to_string())
            }
        })
    }

    fn get_moovables_by_type(moovable_type: String) -> Vec<WitMoovable> {
        with_state(|State(state)| {
            state
                .moovables
                .clone()
                .into_iter()
                .filter(|moovable| moovable.moovable_type == moovable_type)
                .map(into_wit_moovable)
                .collect::<Vec<WitMoovable>>()
        })
    }

    fn add_data(name: String, data: WitData) -> WitMoovableResult {
        with_state(|State(state)| {
            if let Some(moovable) = state
                .moovables
                .iter_mut()
                .find(|moovable| moovable.name == name)
            {
                moovable.data = prepend(moovable.data.clone(), from_wit_data(data));
                WitMoovableResult::Ok(into_wit_moovable(moovable.clone()))
            } else {
                WitMoovableResult::Error("Bike not found".to_string())
            }
        })
    }

    fn get_current(moovable_type: String) -> WitMoovableResult {
        with_state(|State(state)| {
            if let Some(moovable) = state
                .moovables
                .iter()
                .find(|moovable| moovable.current && moovable.moovable_type == moovable_type)
            {
                WitMoovableResult::Ok(into_wit_moovable(moovable.clone()))
            } else {
                WitMoovableResult::Error("No current moovable".to_string())
            }
        })
    }

    fn set_current(name: String) -> WitMoovableResult {
        with_state(|State(state)| {
            if state.moovables.iter().any(|moovable| moovable.name == name) {
                let mut result: WitMoovable = WitMoovable {
                    name: "".to_string(),
                    moovable_type: "".to_string(),
                    current: false,
                    data: Vec::new(),
                };
                state.moovables.iter_mut().for_each(|moovable| {
                    if moovable.name == name {
                        moovable.current = true;
                        result = into_wit_moovable(moovable.clone())
                    } else {
                        moovable.current = false
                    }
                });
                WitMoovableResult::Ok(result)
            } else {
                WitMoovableResult::Error("Bike not found".to_string())
            }
        })
    }

    fn import_moovable(moovable: WitMoovable) -> WitMoovableResult {
        with_state(|State(state)| {
            if state.moovables.iter_mut().any(|m| m.name == moovable.name) {
                WitMoovableResult::Error("Moovable already exists".to_string())
            } else {
                state.moovables.push(from_wit_moovable(moovable.clone()));
                WitMoovableResult::Ok(moovable)
            }
        })
    }

    fn import_all(moovables: WitMoovables) {
        with_state(|State(state)| {
            state.moovables = from_wit_moovables(moovables.moovables);
            state.goal = moovables.goal;
        });
    }

    fn get_trend(name: String, end_date: String) -> WitTrendResult {
        with_state(|State(state)| {
            if let Some(current) = state
                .moovables
                .iter()
                .find(|moovable| moovable.name == name)
            {
                let moovables = state.moovables.clone();
                match MoovingState::trend(moovables, current, state.goal, end_date) {
                    Some(trend) => WitTrendResult::Ok(into_wit_trend(trend)),
                    None => {
                        WitTrendResult::Error("not enough data to do trend analysis".to_string())
                    }
                }
            } else {
                WitTrendResult::Error("Moovable not found".to_string())
            }
        })
    }
}
