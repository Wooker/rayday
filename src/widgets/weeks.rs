use chrono::{Date, Local, Duration, Datelike, Weekday, Month};
use num_traits::FromPrimitive;
use tui::{text::{Text, Spans, Span}, style::{Style, Color}};

#[derive(Debug)]
pub struct Weeks<'a> {
    content: Text<'a>,
}

impl<'a> Weeks<'a> {
    pub fn new(today: Date<Local>, height: u16, width: u16) -> Self {
        let start = today.checked_sub_signed(
                Duration::weeks(height.div_ceil(4).try_into().expect("Could not convert u16 to i64"))
        ).expect("Could not subtract date");

        let a = start.weekday().number_from_monday() as i64 - 1;
        let mut curr_date = start.checked_sub_signed(Duration::days(a)).unwrap();
        let mut curr_month = curr_date.month();
        let mut curr_height = 1; // first month name

        let mut text: Vec<Spans> = Vec::new();
        let mut spans: Vec<Span> = Vec::new();

        let mut highlight_style = Style::default();

        while curr_height != height {
            if curr_date == today {
                highlight_style = Style::default().fg(Color::White).bg(Color::Blue);
            } else {
                highlight_style = Style::default().fg(Color::White);
            }

            spans.push(Span::styled(format!("{:>2}", curr_date.day().to_string()), highlight_style));
            spans.push(Span::raw(" "));

            curr_date = curr_date.succ();


            // Add Month name
            if curr_month != curr_date.month() {
                curr_month = curr_date.month();

                text.push(Spans::from(spans));
                text.push(Spans::from(vec![
                    Span::styled(
                        format!(
                            "{:^width$}",
                            Month::from_u32(curr_month).unwrap().name(),
                            width=(width as usize)
                        ),
                        Style::default().fg(Color::Cyan)
                    )
                ]));
                spans = Vec::new();
                for day in 0..curr_date.weekday().number_from_monday() - 1 {
                    spans.push(Span::raw("   "));
                }

                curr_height += 1;
            } else if curr_date.weekday() == Weekday::Mon {
                text.push(Spans::from(spans));
                spans = Vec::new();
                curr_height += 1;
            }

        }

        Weeks {
            content: Text::from(text),
        }
    }

    pub fn content(self) -> Text<'a> {
        self.content
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    const HEIGHT: u16 = 57;

    #[test]
    fn a() {}
}
