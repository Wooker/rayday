use std::ops::Div;

use crate::{
    app::{App, InputMode},
    widgets::{
        calendar::CalendarWidget,
        event_view::{EventView, EventViewState},
        grid::Grid,
        popup::{self, centered_rect, PopupAdd},
        time_grid::TimeGrid,
        weeks::Weeks,
    },
};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Corner, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Map, Rectangle},
        Block, BorderType, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row,
        StatefulWidget, Table, Tabs, Widget, Wrap,
    },
    Frame,
};

use chrono::prelude::*;
use chrono::{Date, Datelike, Duration, Local, Month, Weekday};
use num_traits::FromPrimitive;
use pickledb::PickleDbIteratorItem;

use crate::event::Event;

pub(crate) fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let size = f.size();

    let chunks = Layout::default()
        .constraints([Constraint::Length(22), Constraint::Min(10)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let info_style = Style::default().fg(Color::Blue);

    let height_without_borders = chunks[0].height - 2;
    let weeks = Weeks::new(app.chosen_date, height_without_borders, chunks[0].width);
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

    let date = app.chosen_date.clone();

    let mut ev = EventView::new(
        app.files.events_list(date),
        &app.input_mode,
        app.enhanced_graphics,
    )
    .block(Block::default().borders(Borders::ALL).title(format!(
        "{} {} {} {:?}",
        date.day(),
        Month::from_u32(date.month()).unwrap().name(),
        date.year(),
        app.chosen_event.selected
    )))
    .style(match app.input_mode {
        InputMode::Selecting => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    })
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(ev, chunks[1], &mut app.chosen_event);

    let popup = PopupAdd::new(&app.input_time, &app.input_description, &app.input_mode).block(
        Block::default()
            .title("Add event")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black)),
    );

    match app.input_mode {
        InputMode::AddingTime => {
            let area = if let Some(block) = &popup.block {
                block.inner(centered_rect(popup::HEIGHT + 2, popup::WIDTH, chunks[1]))
            } else {
                centered_rect(popup::HEIGHT, popup::WIDTH, chunks[1])
            };
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input_time.len() as u16 + 2,
                // Move one line down, from the border to the input line
                area.y + 2, // Title + field name
            );
            f.render_widget(Clear, area); //clear the background
            f.render_widget(popup, area);
        }
        InputMode::AddingDescription => {
            let area = if let Some(block) = &popup.block {
                block.inner(centered_rect(popup::HEIGHT + 2, popup::WIDTH, chunks[1]))
            } else {
                centered_rect(popup::HEIGHT, popup::WIDTH, chunks[1])
            };
            f.set_cursor(area.x + app.input_description.len() as u16 + 2, area.y + 5);
            f.render_widget(Clear, area);
            f.render_widget(popup, area);
        }
        _ => {
            app.input_time = "".to_string();
            app.input_description = "".to_string();
        }
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
