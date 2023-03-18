use std::fmt::Display;

use crate::case::Case;

#[derive(Debug, Clone)]
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

        write!(f, "{}", str);

        write!(
            f,
            "State {{ armed: {}, errored: {}, alarm: {} }}",
            self.armed, self.errored, self.alarm
        )
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
            errored: (was_errored && !armed) || (case.on && triggered),
            alarm: (triggered && was_armed) || (was_alarm && !case.off),
        }
    }
}
