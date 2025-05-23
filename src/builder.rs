use crate::{
    consts::{DAYS, GRID_SIZE, WEEKS},
    state::StateMap,
};
use chrono::Datelike;

pub fn build_dates(origin: Option<chrono::NaiveDate>) -> StateMap {
    let mut map = StateMap::default();
    let then = origin.unwrap_or_else(|| {
        chrono::Local::now().date_naive() - chrono::Duration::days(GRID_SIZE as i64)
    });
    let then = then + chrono::Duration::days(then.weekday().num_days_from_monday() as i64 * -1);

    let mut i = 0;
    for x in &mut map.0 {
        x.0 = (then
            + chrono::Duration::weeks(i % WEEKS as i64)
            + chrono::Duration::days(i / WEEKS as i64 % DAYS as i64 - 1))
        .into();
        x.1 = 0;
        i += 1;
    }

    map
}

pub fn build_grid(grid: StateMap) -> String {
    let mut res = String::from(
        r#"
<!DOCTYPE html>
<html>
<head>
  <style>
    body {
      background-color: white;
    }

    .filled100:hover .tooltip {
      display: block;
    }

    .filled90:hover .tooltip {
      display: block;
    }

    .filled80:hover .tooltip {
      display: block;
    }

    .filled70:hover .tooltip {
      display: block;
    }

    .filled60:hover .tooltip {
      display: block;
    }

    .filled50:hover .tooltip {
      display: block;
    }

    .filled40:hover .tooltip {
      display: block;
    }

    .filled30:hover .tooltip {
      display: block;
    }

    .filled20:hover .tooltip {
      display: block;
    }

    .filled10:hover .tooltip {
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

    .filled10 {
      border: 1px solid #A9A9A9;
      background-color: #A9A9A9;
      height: 10px;
      width: 10px;
    }

    .filled20 {
      border: 1px solid #989898;
      background-color: #989898;
      height: 10px;
      width: 10px;
    }

    .filled30 {
      border: 1px solid #878787;
      background-color: #878787;
      height: 10px;
      width: 10px;
    }

    .filled40 {
      border: 1px solid #767676;
      background-color: #767676;
      height: 10px;
      width: 10px;
    }

    .filled50 {
      border: 1px solid #656565;
      background-color: #656565;
      height: 10px;
      width: 10px;
    }

    .filled60 {
      border: 1px solid #545454;
      background-color: #545454;
      height: 10px;
      width: 10px;
    }

    .filled70 {
      border: 1px solid #434343;
      background-color: #434343;
      height: 10px;
      width: 10px;
    }

    .filled80 {
      border: 1px solid #323232;
      background-color: #323232;
      height: 10px;
      width: 10px;
    }

    .filled90 {
      border: 1px solid #212121;
      background-color: #212121;
      height: 10px;
      width: 10px;
    }

    .filled100 {
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
        if i % WEEKS == 0 {
            res += "<tr>";
        }

        let class = match x.1 {
            10 => "filled100",
            9 => "filled90",
            8 => "filled80",
            7 => "filled70",
            6 => "filled60",
            5 => "filled50",
            4 => "filled40",
            3 => "filled30",
            2 => "filled20",
            1 => "filled10",
            _ => "empty",
        };

        res += &format!(
            r#"<td class="{}"><span class="tooltip">{}</span></td>"#,
            class, x.0
        );

        if i % WEEKS == WEEKS - 1 {
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

pub fn generate_json_grid(state: StateMap) -> String {
    let mut res = String::from("[");

    for x in 0..GRID_SIZE {
        if x % WEEKS == 0 {
            res += "\n";
        }

        res += &format!("{}", state.0[x].1);

        if x < GRID_SIZE - 1 {
            res += ",";
        }
    }
    res += "\n]";
    res
}

pub fn generate_txt_grid(state: StateMap) -> String {
    let mut res = String::new();

    for x in 0..GRID_SIZE {
        if x % WEEKS == 0 && x != 0 {
            res += "\n";
        }

        let mut s = state.0[x].1.to_string();
        if state.0[x].1 == 10 {
            s = "A".to_string();
        }

        res += &s;
    }

    res
}
