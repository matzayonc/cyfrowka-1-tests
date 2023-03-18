use std::{fs::File, io::Write, path::Path};

use case::Case;
use state::State;

mod case;
mod state;

fn main() {
    let path = Path::new("data.dp");
    let mut file = File::create(&path).expect("create error");
    file.write("Data:\n".as_bytes()).unwrap();

    for case in Case::iter() {
        let expected = State::predict(&case);

        let string = format!("{}{}{}", "0".repeat(7), case, expected);
    }

    file.flush().unwrap();
}
