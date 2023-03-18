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
        println!("{}\n", case);
        let expected = State::predict(&case);

        let mut flow = Vec::<Case>::new();
        flow.append(&mut case.before.prepare());
        flow.push(case);
        flow.append(&mut expected.clean());
        flow.push(Case::new()); // padding

        for step in flow {
            println!("STEP {}\n", step);
            let after = State::predict(&step);

            let case_string = format!("{}{}{}\n", "0".repeat(7), step, after);
            file.write(case_string.as_bytes()).unwrap();
        }
    }

    file.flush().unwrap();
}
