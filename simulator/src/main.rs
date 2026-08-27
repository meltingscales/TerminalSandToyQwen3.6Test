//! src/main.rs — Terminal sand simulation with ratatui

mod particles;
mod simulation;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};

use std::io;

use particles::Particle;
use simulation::World;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    let mut world = World::new();

    clear_screen();
    print_help();

    let mut paused = false;

    while terminal.draw(|frame| render(frame, &world)).is_ok() {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                CEvent::Key(evt) => {
                    if evt.kind != KeyEventKind::Press { continue; }

                    match evt.code {
                        KeyCode::Esc | KeyCode::Char('c') if evt.modifiers.contains(KeyModifiers::CONTROL) => {
                            break;
                        },
                        KeyCode::Char('r') => world.reset(),
                        KeyCode::Char('m') => world.toggle_rain(),
                        KeyCode::Char(' ') => paused = !paused,
                        _ => {}
                    }
                }
                CEvent::Mouse(evt) => {
                    match evt.kind {
                        event::MouseEventKind::Down(_) | event::MouseEventKind::Drag(_) => {
                            world.paint_at(evt.column as usize, evt.row as usize);
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        if !paused {
            world.tick();
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn render(frame: &mut ratatui::Frame, world: &World) {
    let size = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(size);

    // Build grid lines with per-cell colored spans.
    let mut lines: Vec<Line> = Vec::with_capacity(world.height());

    for y in 0..world.height() {
        let mut spans = Vec::new();
        for x in 0..world.width() {
            let p = world.get(x, y);
            if p.is_empty() {
                spans.push(Span::raw(" "));
            } else {
                spans.push(Span::styled(
                    format!("{}", p.glyph()),
                    Style::default().fg(p.fg_color()).add_modifier(Modifier::BOLD),
                ));
            }
        }
        lines.push(Line::from(spans));
    }

    // Render the grid as a text block.
    frame.render_widget(
        ratatui::text::Text::from(lines)
            .patch_style(Style::default().bg(Color::Rgb(17, 8, 4))),
        chunks[0],
    );

    let stats = format!(
        "Sand: {:>5} | Water: {:>5} | Seeds: {:>3} | Plants: {:>4} | {}",
        world.count(Particle::Sand),
        world.count(Particle::Water),
        world.count(Particle::Seed),
        world.count(Particle::Plant),
        if world.rain_on() { "Rain ACTIVE" } else { "Rain OFF" },
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::BOTTOM)
            .title(" Terminal Sand Toy — Qwen 3.6 + Hermes ")
            .style(Style::default().fg(Color::White)),
        chunks[1],
    );

    // Render stat line over the bottom chunk area.
    frame.render_widget(
        Line::styled(stats, Style::default().fg(Color::Yellow)),
        chunks[0],
    );
}

fn clear_screen() {
    print!("\x1b[H\x1b[2J");
}

fn print_help() {
    println!("=== Terminal Sand Toy ===");
    println!("  [Space]   Pause / Resume");
    println!("  [R]       Reset world state");
    println!("  [M]       Toggle rain events");
    println!("  [ESC]     Exit");
}
