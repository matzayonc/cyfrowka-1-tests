use std::fmt::Display;

use crate::state::State;

#[derive(Debug, Clone)]
pub struct Case {
    pub on: bool,
    pub off: bool,
    pub bits: u8,

    pub before: State,
}

pub struct Cases {
    pub case: Case,
}

impl Iterator for Cases {
    type Item = Case;

    fn next(self: &mut Cases) -> Option<Self::Item> {
        let c = &mut self.case;

        if !c.on && !c.off {
            c.on = true;
            return Some(c.clone());
        } else {
            c.on = false;
        }

        if !c.off {
            c.off = true;
            return Some(c.clone());
        } else {
            c.off = false;
        }

        if c.bits <= 0xF {
            c.bits += 1;
            return Some(c.clone());
        } else {
            c.bits = 0;
        }

        match c.before {
            State {
                armed: false,
                errored: false,
                alarm: false,
            } => {
                c.before.armed = true;
                return Some(c.clone());
            }
            State {
                armed: true,
                errored: false,
                alarm: false,
            } => {
                c.before.armed = false;
                c.before.errored = true;
                return Some(c.clone());
            }
            State {
                armed: false,
                errored: true,
                alarm: false,
            } => {
                c.before = State {
                    armed: true,
                    errored: false,
                    alarm: true,
                };
                return Some(c.clone());
            }

            _ => return None,
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        for i in 0..4 {
            if self.bits & (1 << i) != 0 {
                str.push('1');
            } else {
                str.push('0');
            }
        }

        if self.off {
            str.push('1');
        } else {
            str.push('0');
        }

        if self.on {
            str.push('1');
        } else {
            str.push('0');
        }

        write!(f, "{}", str)
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

    pub fn iter() -> Cases {
        Cases { case: Case::new() }
    }
}

#[test]
fn test_case() {
    for case in Case::iter() {
        println!("{}\n", case);
    }
}

#[test]
fn test_conflict() {
    println!("test start");
    for case in Case::iter() {
        assert!(!(case.on && case.off));
        assert!(!(case.before.alarm && case.before.errored));

        if case.before.alarm {
            assert!(case.before.armed);
        }
        if case.before.errored {
            assert!(!case.before.armed);
        }
    }
}
