use crate::state::State;

#[derive(Debug, Clone)]
pub struct Case {
    pub on: bool,
    pub off: bool,
    pub bits: u8,

    pub before: State,
}

impl Iterator for Case {
    type Item = Case;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.on {
            return Some(Case { on: true, ..*self });
        } else {
            self.on = false;
        }

        if !self.off {
            return Some(Case { off: true, ..*self });
        } else {
            self.off = false;
        }

        if self.bits < 0xF {
            return Some(Case {
                bits: self.bits + 1,
                ..*self
            });
        } else {
            self.bits = 0;
        }

        if !self.before.armed {
            return Some(Case {
                before: State {
                    armed: true,
                    ..self.before
                },
                ..*self
            });
        } else {
            self.before.armed = false;
        }

        if !self.before.errored {
            return Some(Case {
                before: State {
                    errored: true,
                    ..self.before
                },
                ..*self
            });
        } else {
            self.before.errored = false;
        }

        if !self.before.alarm {
            return Some(Case {
                before: State {
                    alarm: true,
                    ..self.before
                },
                ..*self
            });
        } else {
            self.before.alarm = false;
        }

        None
    }
}

impl Case {
    pub fn new() -> Self {
        Case {
            on: false,
            off: false,
            bits: 0,
            before: State {
                armed: false,
                errored: false,
                alarm: false,
            },
        }
    }

    pub fn reset() -> String {
        String::from("00000010\n")
    }

    pub fn load(str: String) -> Case {
        let mut case = Case::new();

        let mut digits = str.chars();

        for i in 0..4 {
            let bit = digits.next().unwrap();
            if bit == '1' {
                case.bits |= 1 << i;
            }
        }

        case.off = digits.next().unwrap() == '1';
        case.on = digits.next().unwrap() == '1';

        if case.next().is_some() {
            panic!("too many digits");
        }

        case
    }
}

#[test]
fn test_case() {
    let mut case = Case::new();

    for i in 0..=F {
        let this_case = Case::load().unwrap();
        case = this_case;

        let case_string = case.data();

        println!("{}", case_string);
    }

    println!()
}
