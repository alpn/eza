use crate::output::cell::TextCell;
use crate::output::time::TimeFormat;

use ansiterm::Style;
use chrono::prelude::*;


pub trait Render {
    fn render(self, style: Style, time_offset: FixedOffset, time_format: TimeFormat) -> TextCell;
}

impl Render for Option<NaiveDateTime> {
    fn render(self, style: Style, time_offset: FixedOffset, time_format: TimeFormat) -> TextCell {
        let datestamp = if let Some(time) = self {
            time_format.format(&DateTime::<FixedOffset>::from_naive_utc_and_offset(time, time_offset))
        } else {
            String::from("-")
        };

        TextCell::paint(style, datestamp)
    }
}
