use std::ops::Div;

use crate::{
    app::{App, InputMode},
    widgets::{
        calendar::CalendarWidget,
        event_view::{EventViewState, EventViewWidget},
        grid::Grid,
        popup::{self, centered_rect, PopupWidget},
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
use chrono::{Datelike, Duration, Local, Month, Weekday};
use num_traits::FromPrimitive;

use crate::event::Event;

pub(crate) fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .state_tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Gray))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.state_tabs.index);
    f.render_widget(tabs, chunks[0]);

    match app.state_tabs.index {
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
    let weeks = Weeks::new(
        app.state_calendar.get_selected_date(),
        height_without_borders,
        chunks[0].width,
    );
    let mut calendar = CalendarWidget::default()
        .with_weeks(weeks)
        .style(&app.input_mode.current().unwrap())
        .highlight_style(app.files.get_config().highlight_color);
    f.render_stateful_widget(calendar, chunks[0], &mut app.state_calendar);

    let selected_date = app.state_calendar.get_selected_date();
    app.state_events.events = app.files.get_events_on_date(selected_date);

    let mut ev = EventViewWidget::new(
        // app.files.get_events_on_date(app.state_calendar.get_selected_date()),
        app.state_events.events.clone(),
        app.enhanced_graphics,
    )
    .block(Block::default().borders(Borders::ALL).title(format!(
        "{} {} {}{}",
        selected_date.day(),
        Month::from_u32(selected_date.month()).unwrap().name(),
        selected_date.year(),
        if let Some(i) = app.state_events.selected {
            format!(
                " - {}",
                app.state_events.events.iter().nth(i).unwrap().desc()
            )
        } else {
            String::new()
        }
    )))
    .style(match app.input_mode.current() {
        Some(InputMode::Select) => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    })
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(ev, chunks[1], &mut app.state_events);

    let popup = PopupWidget::new().block(
        Block::default()
            .title("Add event")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black)),
    );

    let popup_area = if let Some(block) = &popup.block {
        block.inner(centered_rect(popup::HEIGHT + 2, popup::WIDTH, chunks[1]))
    } else {
        centered_rect(popup::HEIGHT, popup::WIDTH, chunks[1])
    };
    if app.state_popup.visible {
        f.render_widget(Clear, popup_area); //clear the background
        f.render_stateful_widget(popup, popup_area, &mut app.state_popup);
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
