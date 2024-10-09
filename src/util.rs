pub mod util {
    macro_rules! format_time_taken {
        ( $time_taken:expr ) => {{
            let nanoseconds: u128 = $time_taken.as_nanos();
            let seconds: u128 = nanoseconds / 1e9 as u128;
            if seconds == 0 {
                let milliseconds: u128 = nanoseconds / 1e6 as u128;
                if milliseconds == 0 {
                    format!("{} microseconds", $time_taken.as_micros())
                } else {
                    format!("{} milliseconds", milliseconds)
                }
            } else {
                format!("{} seconds", seconds)
            }
        }};
    }

    pub(crate) use format_time_taken;
}
