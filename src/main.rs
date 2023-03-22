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

        let mut simple = Vec::<Case>::new();
        simple.push(Case::new());
        simple.append(&mut case.before.prepare());
        simple.push(case);
        simple.append(&mut expected.clean());
        simple.push(Case::new()); // padding

        let mut flow = Vec::<Case>::new();

        let mut previous = Case::new();

        for i in simple {
            flow.append(&mut i.between(&previous));
            flow.push(i.clone());
            flow.push(i.clone());
            flow.push(i.clone());
            flow.push(i.clone());
            flow.push(i.clone());
            previous = i.clone();
        }

        for step in flow {
            println!("STEP {}\n", step);
            let after = State::predict(&step);

            // let case_string = format!("{}{}{}\n", "0".repeat(7), step, after);
            let case_string = format!("{:#x}\n", (step.num() as u16) * 8 + (after.num() as u16));
            file.write(case_string[2..].as_bytes()).unwrap();
        }
    }

    file.flush().unwrap();
}
