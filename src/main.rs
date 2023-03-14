use std::{fs::File, io::Write, path::Path};

struct Case {
    pub on: bool,
    pub bits: u8,
}

impl Case {
    pub fn new() -> Self {
        Case { on: false, bits: 0 }
    }

    /// 0 on
    /// 1 off
    /// 4x czujniki

    pub fn next(mut c: Self) -> Option<Self> {
        if c.on {
            c.on = !c.on;
        } else {
            c.bits += 1;
            c.on = false;
        };

        if c.bits & 0xF0 > 0 {
            return None;
        } else {
            return Some(c);
        }
    }

    pub fn data(&self) -> String {
        let mut data = String::new();
        data.push_str("00");

        let mut mask = 1;

        for _ in 0..4 {
            let bit = (self.bits & mask) > 0;
            data.push_str(if bit { "1" } else { "0" });
            mask <<= 1;
        }

        data.push_str(if self.on { "01" } else { "10" });
        data.push_str("\n");
        data
    }

    pub fn reset() -> String {
        String::from("00000010\n")
    }
}

fn main() {
    let path = Path::new("data.dp");
    let mut file = File::create(&path).expect("create error");
    file.write("Data:\n".as_bytes()).unwrap();

    let mut case = Case::new();

    while let Some(this_case) = Case::next(case) {
        case = this_case;

        let case_string = case.data();

        file.write(case_string.as_bytes()).unwrap();
        file.write(Case::reset().as_bytes()).unwrap();
    }

    file.flush().unwrap();
}
