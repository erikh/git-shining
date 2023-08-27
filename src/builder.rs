use crate::{
    consts::{DAYS, GRID_SIZE, WEEKS},
    state::StateMap,
};
use chrono::Datelike;

pub fn build_dates() -> StateMap {
    let mut map = StateMap::default();
    let now = chrono::Local::now().date_naive();
    let now = now + chrono::Duration::days(now.weekday().num_days_from_monday() as i64 - 1)
        - chrono::Duration::days(GRID_SIZE as i64);

    let mut i = 0;
    for x in &mut map.0 {
        x.0 = now
            + chrono::Duration::weeks(i % WEEKS as i64)
            + chrono::Duration::days(i / WEEKS as i64 % DAYS as i64 - 1);
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

    .filled100 {
      border: 1px solid black;
      background-color: black;
      height: 10px;
      width: 10px;
    }

    .filled10 {
      border: 1px solid #EEEEEE;
      background-color: #EEEEEE;
      height: 10px;
      width: 10px;
    }

    .filled20 {
      border: 1px solid #CCCCCC;
      background-color: #CCCCCC;
      height: 10px;
      width: 10px;
    }

    .filled30 {
      border: 1px solid #AAAAAA;
      background-color: #AAAAAA;
      height: 10px;
      width: 10px;
    }

    .filled40 {
      border: 1px solid #999999;
      background-color: #999999;
      height: 10px;
      width: 10px;
    }

    .filled50 {
      border: 1px solid #777777;
      background-color: #777777;
      height: 10px;
      width: 10px;
    }

    .filled60 {
      border: 1px solid #666666;
      background-color: #666666;
      height: 10px;
      width: 10px;
    }

    .filled70 {
      border: 1px solid #444444;
      background-color: #444444;
      height: 10px;
      width: 10px;
    }

    .filled80 {
      border: 1px solid #333333;
      background-color: #333333;
      height: 10px;
      width: 10px;
    }

    .filled90 {
      border: 1px solid #222222;
      background-color: #222222;
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

pub fn generate_json_grid() -> String {
    let mut res = String::from("[");

    for x in 0..GRID_SIZE {
        if x % WEEKS == 0 {
            res += "\n";
        }
        res += r#"0"#;
        if x < GRID_SIZE - 1 {
            res += ",";
        }
    }
    res += "\n]";
    res
}
