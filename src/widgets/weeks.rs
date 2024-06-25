use std::ops::Div;

use chrono::{Datelike, Duration, Local, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;
use rayday::get_days_from_month;
use tui::{
    style::{Color, Style},
    text::{Span, Spans, Text},
};

#[derive(Debug)]
pub struct Weeks<'a> {
    content: Text<'a>,
}

impl<'a> Weeks<'a> {
    pub fn get_curr_date(
        today: NaiveDate, /*Date<Local>*/
        height: u16,
        width: u16,
    ) -> NaiveDate /*Date<Local>*/ {
        let mut start = today
            .checked_sub_signed(Duration::weeks(height.div(4).into()))
            .expect("Could not subtract date");

        let month_diff = today
            .month()
            .checked_sub(start.month())
            .unwrap_or(12 - start.month() + today.month());

        start
            .checked_add_signed(Duration::weeks(month_diff.into()))
            .expect("Could not add date");

        let a = start.weekday().number_from_monday() as i64 - 1;
        start.checked_sub_signed(Duration::days(a)).unwrap()
    }

    pub fn new(today: NaiveDate /*Date<Local>*/, height: u16, width: u16) -> Self {
        let mut curr_date = Self::get_curr_date(today, height, width);
        let mut curr_month = curr_date.month();
        let mut curr_height = 0; // first month name

        let mut text: Vec<Spans> = Vec::new();
        let mut spans: Vec<Span> = Vec::new();

        let mut highlight_style = Style::default();

        if curr_date
            .checked_add_signed(Duration::days(7))
            .unwrap()
            .month()
            != curr_month
        {
            // Set the current date to the first day of the month
            curr_date = curr_date
                .checked_add_signed(Duration::days(
                    get_days_from_month(curr_date.year(), curr_month) - curr_date.day0() as i64,
                ))
                .unwrap();
        }

        while curr_height != height {
            if curr_date.month() != curr_month {
                curr_month = curr_date.month();
                spans.push(Span::styled(
                    format!(
                        "{:^width$}",
                        Month::from_u32(curr_month).unwrap().name(),
                        width = (width as usize)
                    ),
                    Style::default().fg(Color::Cyan),
                ));
                text.push(Spans::from(spans));
                curr_height += 1;

                spans = Vec::new();
                continue;
            }

            while (curr_date.weekday() != Weekday::Mon || spans.len() == 0)
                && curr_month == curr_date.month()
            {
                if curr_date == today {
                    highlight_style = Style::default().fg(Color::White).bg(Color::Blue);
                } else {
                    highlight_style = Style::default().fg(Color::White);
                }

                if curr_date.day0() == 0 {
                    // add empty space to match weekday column
                    for day in 0..curr_date.weekday().number_from_monday() - 1 {
                        spans.push(Span::raw("   "));
                    }
                }

                // Add the day in the spans
                spans.push(Span::styled(
                    format!("{:>2}", curr_date.day().to_string()),
                    highlight_style,
                ));
                // Add spance between days
                spans.push(Span::raw(" "));

                // Increase date
                curr_date = curr_date.succ_opt().expect("Last date is reached");
            }
            text.push(Spans::from(spans));
            curr_height += 1;

            spans = Vec::new();
        }

        Weeks {
            content: Text::from(text),
        }
    }

    pub fn content(self) -> Text<'a> {
        self.content
    }
}
