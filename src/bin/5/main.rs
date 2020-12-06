use std::fmt::Display;

mod input;
fn main() {
    let codes: Vec<SeatCode> = input::INPUT
        .split("\n")
        .map(|code| SeatCode { code })
        .collect();

    let mut seat_ids = codes
        .iter()
        .map(|code| code.seat_id())
        .collect::<Vec<u32>>();
    seat_ids.sort();
    dbg!(seat_ids);
}

#[derive(Debug)]
struct SeatCode {
    code: &'static str,
}

impl Display for SeatCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: row {}, column {}, seat ID {}",
            self.code,
            self.row(),
            self.column(),
            self.seat_id()
        )
    }
}
impl SeatCode {
    fn column(&self) -> u32 {
        self.column_calc(&self.code[7..])
    }

    fn row(&self) -> u32 {
        self.row_calc(&self.code[0..6])
    }

    fn seat_id(&self) -> u32 {
        self.row() * 8 + self.column()
    }

    fn row_calc(&self, code: &str) -> u32 {
        match code.chars().nth(0) {
            Some('F') => self.row_calc(&code[1..]),
            Some('B') => (2_u32.pow(code.len() as u32 + 1) / 2) + self.row_calc(&code[1..]),
            None => 0,
            _ => unreachable!("asdf{}", code.chars().nth(0).unwrap()),
        }
    }

    fn column_calc(&self, code: &str) -> u32 {
        match code.chars().nth(0) {
            Some('L') => self.column_calc(&code[1..]),
            Some('R') => (2_u32.pow(code.len() as u32) / 2) + self.column_calc(&code[1..]),
            None => 0,
            _ => unreachable!(),
        }
    }
}
