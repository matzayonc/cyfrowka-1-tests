use std::fmt::Display;

use crate::case::Case;

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub armed: bool,
    pub errored: bool,
    pub alarm: bool,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        str.push(if self.armed { '1' } else { '0' });
        str.push(if self.errored { '1' } else { '0' });
        str.push(if self.alarm { '1' } else { '0' });

        write!(f, "{}", str)
    }
}

impl State {
    pub fn predict(case: &Case) -> Self {
        let triggered = case.bits != 0;

        let State {
            armed: was_armed,
            errored: was_errored,
            alarm: was_alarm,
        } = case.before;

        let armed = (was_armed && !case.off) || (case.on && !triggered);

        State {
            armed,
            errored: (was_errored && !armed) || (case.on && triggered && !armed),
            alarm: (triggered && was_armed) || (was_alarm && !case.off),
        }
    }

    pub fn prepare(&self) -> Vec<Case> {
        let mut v = vec![];
        if self.errored {
            v.push(Case {
                bits: 0xF,
                on: true,
                ..Case::new()
            });
        } else if self.armed {
            v.push(Case {
                on: true,
                ..Case::new()
            });
        }
        if self.alarm {
            v.push(Case {
                bits: 0xF,
                ..Case::new()
            });
        }

        v
    }

    pub fn clean(&self) -> Vec<Case> {
        let mut v = vec![];
        if self.armed || self.alarm {
            v.push(Case {
                bits: 0,
                off: true,
                ..Case::new()
            });
        }
        if self.errored {
            v.push(Case {
                on: true,
                ..Case::new()
            });
            v.push(Case {
                bits: 0,
                off: true,
                ..Case::new()
            });
        }

        v
    }

    pub fn num(&self) -> u8 {
        let mut num = 0;
        if self.armed {
            num += 1;
        }
        if self.errored {
            num += 2;
        }
        if self.alarm {
            num += 4;
        }
        num
    }
}

#[test]
fn test() {
    let state = State {
        armed: false,
        errored: true,
        alarm: false,
    };
    assert_eq!(
        State::prepare(&state)[0],
        Case {
            bits: 0xF,
            on: true,
            ..Case::new()
        }
    );
}
