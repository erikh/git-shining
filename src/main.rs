use anyhow::anyhow;
use chrono::Datelike;
use serde_derive::{Deserialize, Serialize};

const WEEKS: usize = 52;
const GRID_SIZE: usize = (WEEKS + 1) * 7;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config(Vec<String>);

impl Config {
    fn valid(&self) -> Result<(), anyhow::Error> {
        if self.0.len() == GRID_SIZE {
            return Ok(());
        }

        Err(anyhow!("Invalid Size: must be {}", GRID_SIZE))
    }

    fn to_grid(&self) -> Result<StateMap, anyhow::Error> {
        self.valid()?;
        let mut dates = build_dates();

        let mut i = 0;
        for x in &mut dates.0 {
            x.1 = self.0[i].trim() == "t";
            i += 1;
        }

        Ok(dates)
    }
}

#[derive(Debug, Clone, Default)]
struct State(chrono::NaiveDate, bool);

#[derive(Debug)]
struct StateMap(Vec<State>);

impl<'a> Default for StateMap {
    fn default() -> Self {
        let mut v = Vec::new();
        for _ in 0..GRID_SIZE {
            v.push(State::default())
        }

        Self(v)
    }
}

#[allow(dead_code)]
fn generate_json_grid() -> String {
    let mut res = String::from("[");
    for y in 0..7 {
        res += "\n";
        for x in 0..(WEEKS + 1) {
            res += r#""f""#;

            if (y * x) < GRID_SIZE - 1 {
                res += ",";
            }
        }
    }
    res += "\n]";
    res
}

fn build_dates() -> StateMap {
    let mut map = StateMap::default();
    let now = chrono::Local::now().date_naive();
    let now = now + chrono::Duration::days(now.weekday().num_days_from_monday() as i64 - 1);

    for y in 0..7 {
        for x in 0..(WEEKS + 1) {
            map.0[x * 7 + y] = State(
                now - chrono::Duration::days(GRID_SIZE as i64)
                    + chrono::Duration::weeks(x as i64)
                    + chrono::Duration::days(y as i64 + 1),
                false,
            );
        }
    }

    map
}

fn build_grid(grid: StateMap) -> String {
    let mut res = String::from(
        r#"
<!DOCTYPE html>
<html>
<head>
  <style>
    .filled:hover .tooltip {
      display: block;
    }

    .empty:hover .tooltip {
      display: block;
    }

    .tooltip {
      display: none;
      background: #C8C8C8;
      margin-left: 28px;
      padding: 10px;
      position: absolute;
      z-index: 1000;
      width: auto;
      height: 1.5em;
    }

    .filled {
      border: 1px solid black;
      background-color: black;
      height: 10px;
      width: 10px;
    }

    .empty {
      border: 1px solid black;
      height: 10px;
      width: 10px;
    }
  </style>
</head>
  <body>
    <table>
        "#,
    );

    let mut i = 0;
    for x in &grid.0 {
        if i % (WEEKS + 1) == 0 {
            res += "<tr>";
        }

        if x.1 {
            res += &format!(
                r#"<td class="filled"><span class="tooltip">{}</span></td>"#,
                x.0
            );
        } else {
            res += &format!(
                r#"<td class="empty"><span class="tooltip">{}</span></td>"#,
                x.0
            );
        }

        if i % (WEEKS + 1) == WEEKS {
            res += "</tr>";
        }

        i += 1;
    }

    res += r"
        </table>
    </body>
</html>
        ";

    res
}

fn main() -> Result<(), anyhow::Error> {
    let res: Config = serde_json::from_str(&std::fs::read_to_string("config.json")?)?;
    println!("{}", build_grid(res.to_grid()?));
    //    println!("{}", generate_json_grid());
    Ok(())
}
