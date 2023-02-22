use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Clock{
            minutes: Self::to_clock_minutes((60*hours)+minutes),
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.minutes = Self::to_clock_minutes(&self.minutes + minutes);
        self
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % 60
    }
    
    pub fn hours(&self) -> i32 {
        self.minutes / 60    
    }

    pub fn to_clock_minutes(minutes: i32) -> i32 {
        (1440 + (minutes%1440))%1440
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
