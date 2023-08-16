use crate::exports::mooving::moovables::api::{
    Data as WitData, Moovable as WitMoovable, Trend as WitTrend,
};

use mooving::{Data, Moovable, Trend};

pub fn from_wit_data(data: WitData) -> Data {
    Data {
        date: data.date.clone(),
        km: data.km,
    }
}

pub fn into_wit_data(data: Data) -> WitData {
    WitData {
        date: data.date,
        km: data.km,
    }
}

pub fn from_wit_moovable(moovable: WitMoovable) -> Moovable {
    Moovable {
        name: moovable.name.clone(),
        moovable_type: moovable.moovable_type.clone(),
        current: moovable.current,
        data: moovable.data.into_iter().map(from_wit_data).collect(),
    }
}

pub fn into_wit_moovable(moovable: Moovable) -> WitMoovable {
    WitMoovable {
        name: moovable.name,
        moovable_type: moovable.moovable_type,
        current: moovable.current,
        data: moovable.data.into_iter().map(into_wit_data).collect(),
    }
}

pub fn from_wit_moovables(moovable: Vec<WitMoovable>) -> Vec<Moovable> {
    moovable
        .into_iter()
        .map(from_wit_moovable)
        .collect::<Vec<Moovable>>()
}

pub fn into_wit_trend(trend: Trend) -> WitTrend {
    WitTrend {
        average: trend.average,
        days: trend.days,
        average_to_reach_goal: trend.average_to_reach_goal,
        goal: trend.goal,
        total: trend.total,
        this_period: trend.this_period,
        moovable_name: trend.moovable_name,
    }
}
