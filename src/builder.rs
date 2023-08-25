use crate::{
    consts::{GRID_SIZE, WEEKS},
    state::{State, StateMap},
};
use chrono::Datelike;

pub fn build_dates() -> StateMap {
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
