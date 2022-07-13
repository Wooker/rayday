use crate::{
    app::{App, InputMode},
    widgets::{
        calendar::CalendarWidget,
        event_view::EventView,
        grid::Grid,
        popup::{centered_rect, PopupAdd},
        time_grid::TimeGrid,
        weeks::Weeks,
    },
};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Map, Rectangle},
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, StatefulWidget,
        Table, Tabs, Widget, Wrap,
    },
    Frame,
};

use chrono::prelude::*;
use num_traits::FromPrimitive;
use pickledb::PickleDbIteratorItem;

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
    let size = f.size();

    let chunks = Layout::default()
        .constraints([Constraint::Length(24), Constraint::Min(10)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let info_style = Style::default().fg(Color::Blue);

    let weeks = Weeks::new(app.chosen_date, chunks[0].height - 10, chunks[0].width);
    let mut calendar = CalendarWidget::new(weeks, app.chosen_date, &app.input_mode)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Calendar Widget"),
        )
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .highlight_style(
            Style::default()
                .bg(app.files.config.color)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(calendar, chunks[0], &mut app.chosen_date);

    let date = Local.ymd(
        app.chosen_date.year(),
        app.chosen_date.month(),
        app.chosen_date.day(),
    );
    let ev = EventView::new(
        app.files.events_list(date),
        &app.input_mode,
        app.enhanced_graphics,
    )
    .block(Block::default().borders(Borders::ALL).title(format!(
        "{} {} {}",
        date.day(),
        Month::from_u32(date.month()).unwrap().name(),
        date.year()
    )))
    .style(match app.input_mode {
        InputMode::Selecting => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    })
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(ev, chunks[1], &mut app.chosen_event);

    let popup = PopupAdd::new(&app.input_time, &app.input_description, &app.input_mode).block(
        Block::default()
            .title("Popup")
            .borders(Borders::ALL)
            .border_type(tui::widgets::BorderType::Thick),
    );

    match app.input_mode {
        InputMode::AddingTime => {
            let area = centered_rect(40, 11, chunks[1]);
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input_time.len() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            );
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(popup, area);
            //f.render_widget(popup, area)
        }
        InputMode::AddingDescription => {
            let area = centered_rect(40, 11, chunks[1]);
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input_description.len() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 4,
            );
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(popup, area);
            //f.render_widget(popup, area)
        }
        _ => {}
    }
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
