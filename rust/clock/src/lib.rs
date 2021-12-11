use std::fmt;

const HRS: i32 = 24;
const MTS: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hrs: i32,
    mts: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hrs, self.mts)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clk = Clock { hrs: 0, mts: 0 };
        clk.process_inputs(hours, minutes);
        clk
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut clk = Clock {
            hrs: self.hrs,
            mts: self.mts,
        };
        clk.process_inputs(0, minutes);
        clk
    }

    fn process_inputs(&mut self, hours: i32, minutes: i32) {
        let mut hours = hours;
        let mut minutes = minutes;

        while minutes < 0 {
            minutes += MTS;
            hours -= 1;
        }

        while hours < 0 {
            hours += HRS;
        }

        self.hrs += hours + (self.mts + minutes) / MTS;
        self.hrs %= HRS;
        self.mts += minutes;
        self.mts %= MTS;
    }
}
