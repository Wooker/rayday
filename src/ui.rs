use crate::{
    app::App,
    widgets::calendar::{List as DayList, ListItem as DayListItem},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap, Table, Row, Cell, StatefulWidget, ListState},
    buffer::Buffer,
    Frame,
};

use pickledb::PickleDbIteratorItem;
use chrono::prelude::*;

use crate::event::Event;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Gray))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_test<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Min(8), Constraint::Length(7)].as_ref())
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Length(5), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        draw_text(f, chunks[0]);
    }
    draw_text(f, chunks[1]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(24), Constraint::Min(10)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let info_style = Style::default().fg(Color::Blue);

    let mut days = DayList::new(vec![
            DayListItem::new("1"),
            DayListItem::new("2"),
            DayListItem::new("3"),
            DayListItem::new("4"),
            DayListItem::new("5"),
            DayListItem::new("6"),
            DayListItem::new("7"),
            DayListItem::new("8"),
            DayListItem::new("9"),
            DayListItem::new("10"),
            DayListItem::new("11"),
            DayListItem::new("12"),
            DayListItem::new("13"),
            DayListItem::new("14"),
            DayListItem::new("15"),
        ])
        .block(Block::default().borders(Borders::ALL).title("Calendar"))
        .highlight_style(match app.files.config.color.as_str() {
            "reset" => Style::default().bg(Color::Reset),
            "black" => Style::default().bg(Color::Black),
            "red" => Style::default().bg(Color::Red),
            "green" => Style::default().bg(Color::Green),
            "yellow" => Style::default().bg(Color::Yellow),
            "blue" => Style::default().bg(Color::Blue),
            "magenta" => Style::default().bg(Color::Magenta),
            "cyan" => Style::default().bg(Color::Cyan),
            "gray" => Style::default().bg(Color::Gray),
            "dark gray" => Style::default().bg(Color::DarkGray),
            "light red" => Style::default().bg(Color::LightRed),
            "light green" => Style::default().bg(Color::LightGreen),
            "light yellow" => Style::default().bg(Color::LightYellow),
            "light blue" => Style::default().bg(Color::LightBlue),
            "light magenta" => Style::default().bg(Color::LightMagenta),
            "light cyan" => Style::default().bg(Color::LightCyan),
            "white" => Style::default().bg(Color::White),
            //Rgb(u8, u8, u8),
            //Indexed(u8),
            _ => Style::default().add_modifier(Modifier::BOLD)
        });
    f.render_stateful_widget(days, chunks[0], &mut app.days_state);

    let mut events: Vec<ListItem> = app
        .events
        .items
        .iter()
        .map(|itm| {
            let s = info_style;
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}|{}: ", itm.time().start_datetime(), itm.time().end_datetime()), s),
                Span::raw(itm.desc()),
            ])];
            ListItem::new(content)
        })
        .collect();
    let events = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Events"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(events, chunks[1], &mut app.events.state);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = vec![Constraint::Percentage(100)];
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);

            // Draw logs
            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);


        }
    }
}
