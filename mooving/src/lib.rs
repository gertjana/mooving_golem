use chrono::naive::NaiveDate;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Data {
    pub date: String,
    pub km: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Moovable {
    pub name: String,
    pub moovable_type: String,
    pub current: bool,
    pub data: Vec<Data>,
}

impl Moovable {
    pub fn new(name: String, moovable_type: String) -> Self {
        Moovable {
            name,
            moovable_type,
            current: false,
            data: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trend {
    pub average: f64,
    pub days: u32,
    pub average_to_reach_goal: f64,
    pub goal: u32,
    pub total: u32,
    pub this_period: u32,
    pub moovable_name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Moovables {
    pub goal: u32,
    pub moovables: Vec<Moovable>,
}

pub struct MoovingState {
    pub goal: u32,
    pub moovables: Vec<Moovable>,
}

pub const NEW_MOOVING_STATE: MoovingState = MoovingState {
    goal: 0,
    moovables: Vec::new(),
};

impl MoovingState {
    fn get_total_other_moovables(moovables: &[Moovable], moovable_type: String) -> f64 {
        let mut sum = 0.0;
        for m in moovables.iter() {
            if m.moovable_type == moovable_type && !m.current {
                for d in m.data.iter() {
                    sum += d.km as f64;
                }
            }
        }
        sum
    }

    fn diff_string_date(d1: &str, d2: &str) -> i64 {
        let date1 = NaiveDate::parse_from_str(d1, "%Y-%m-%d").unwrap();
        let date2 = NaiveDate::parse_from_str(d2, "%Y-%m-%d").unwrap();
        date2.signed_duration_since(date1).num_days()
    }

    fn get_deltas(moovable: &Moovable) -> Vec<f64> {
        let mut deltas: Vec<f64> = Vec::new();
        let mut prev: Option<&Data> = None;
        for data in &moovable.data {
            if let Some(prev_data) = prev {
                let diff: f64 = (prev_data.km - data.km) as f64;
                let days = Self::diff_string_date(&prev_data.date, &data.date);
                let delta = diff / days as f64;
                deltas.push(delta.abs());
            }
            prev = Some(data);
        }
        deltas
    }

    pub fn trend(
        moovables: Vec<Moovable>,
        current: &Moovable,
        goal: u32,
        end_date: String,
    ) -> Option<Trend> {
        let deltas = Self::get_deltas(current);
        if deltas.len() <= 1 {
            None
        } else {
            let offset: f64 =
                Self::get_total_other_moovables(&moovables, current.moovable_type.clone());
            let last_date = &current.data[0].date;
            let last_value = current.data[0].km as f64;

            let avg = deltas.iter().sum::<f64>() / deltas.len() as f64;
            let days = Self::diff_string_date(last_date, &end_date) as f64;
            let total = (avg * days + last_value + offset).round() as u32;
            let avg_goal = if total >= goal {
                0.0
            } else {
                (goal - total) as f64 / days
            };
            let this_year = (avg * days) as u32;
            println!("offset: {offset} avg: {avg}, days: {days}, total: {total}, this_year: {this_year}, goal: {goal}, avg_goal: {avg_goal}");
            Some(Trend {
                average: avg,
                days: days as u32,
                total,
                this_period: this_year,
                goal,
                average_to_reach_goal: avg_goal,
                moovable_name: current.name.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_moovables() -> Vec<Moovable> {
        vec![
            Moovable {
                name: "test".to_string(),
                moovable_type: "bike".to_string(),
                current: true,
                data: vec![
                    Data {
                        date: "2023-08-03".to_string(),
                        km: 200,
                    },
                    Data {
                        date: "2023-08-02".to_string(),
                        km: 100,
                    },
                    Data {
                        date: "2023-08-01".to_string(),
                        km: 0,
                    },
                ],
            },
            Moovable {
                name: "test2".to_string(),
                moovable_type: "bike".to_string(),
                current: false,
                data: vec![Data {
                    date: "2023-08-05".to_string(),
                    km: 100,
                }],
            },
            Moovable {
                name: "test3".to_string(),
                moovable_type: "bike".to_string(),
                current: false,
                data: vec![Data {
                    date: "2023-08-05".to_string(),
                    km: 100,
                }],
            },
            Moovable {
                name: "test4".to_string(),
                moovable_type: "car".to_string(),
                current: true,
                data: vec![Data {
                    date: "2023-08-05".to_string(),
                    km: 100,
                }],
            },
        ]
    }

    #[test]
    fn test_date_diff() {
        let d1 = "2023-08-01";
        let d2 = "2023-08-10";
        assert_eq!(MoovingState::diff_string_date(d1, d2), 9);
    }

    #[test]
    fn test_trend_successful() {
        let moovables = test_moovables().to_owned();
        let current = &moovables[0].clone();
        let goal = 1000;
        let end_date = "2023-08-10".to_string();

        let trend = MoovingState::trend(moovables, current, goal, end_date).unwrap();

        assert_eq!(trend.average, 100.0);
        assert_eq!(trend.days, 7);
        assert_eq!(trend.total, 1100);
        assert_eq!(trend.this_period, 700);
        assert_eq!(trend.goal, 1000);
        assert_eq!(trend.average_to_reach_goal, 0.0);
        assert_eq!(trend.moovable_name, "test");
    }

    #[test]
    fn test_trend_failed() {
        let moovables = test_moovables().to_owned();
        let current = &moovables[3].clone(); // with only one data point
        let goal = 1000;
        let end_date = "2023-08-10".to_string();

        let trend = MoovingState::trend(moovables, current, goal, end_date);

        assert!(trend.is_none());
    }

    #[test]
    fn test_with_file() {
        let file = std::fs::read_to_string("../import/moofing.json").expect("Unable to read file");
        let moovables: Moovables =
            serde_json::from_str(&file).expect("JSON was not well-formatted");

        assert_eq!(moovables.goal, 12000);
        assert_eq!(moovables.moovables.len(), 5);

        let current = &moovables.moovables[0].clone();
        let goal = 12000;
        let end_date = "2023-08-10".to_string();

        let trend = MoovingState::trend(moovables.moovables.clone(), current, goal, end_date);
        assert!(trend.is_some());
    }

    #[test]
    fn test_goal_not_reached() {
        let moovables = test_moovables().to_owned();
        let current = &moovables[0].clone();
        let goal = 2000;
        let end_date = "2023-08-10".to_string();

        let trend = MoovingState::trend(moovables, current, goal, end_date).unwrap();

        assert_eq!(trend.average, 100.0);
        assert_eq!(trend.average_to_reach_goal, 128.57142857142858);
    }
}
