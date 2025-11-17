// Terminal UI using ratatui

use crate::monitor::ZmkMonitor;
use crate::protocol::{LogLevel, ConnectionType};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

pub struct App {
    monitor: Arc<Mutex<ZmkMonitor>>,
    selected_tab: usize,
    should_quit: bool,
    scroll_offset: usize,
}

impl App {
    pub fn new(monitor: Arc<Mutex<ZmkMonitor>>) -> Self {
        Self {
            monitor,
            selected_tab: 0,
            should_quit: false,
            scroll_offset: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app
        let res = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            eprintln!("Error: {}", err);
        }

        Ok(())
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut tick_interval = interval(Duration::from_millis(100));

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    terminal.draw(|f| self.draw(f))?;
                }
            }

            // Handle input (non-blocking)
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('1') => self.selected_tab = 0,
                        KeyCode::Char('2') => self.selected_tab = 1,
                        KeyCode::Char('3') => self.selected_tab = 2,
                        KeyCode::Char('4') => self.selected_tab = 3,
                        KeyCode::Up => {
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                        KeyCode::Down => self.scroll_offset += 1,
                        KeyCode::PageUp => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(10);
                        }
                        KeyCode::PageDown => {
                            self.scroll_offset += 10;
                        }
                        _ => {}
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Status bar
            ])
            .split(size);

        // Draw title
        self.draw_title(f, chunks[0]);

        // Draw tabs
        self.draw_tabs(f, chunks[1]);

        // Draw content based on selected tab
        match self.selected_tab {
            0 => self.draw_overview(f, chunks[2]),
            1 => self.draw_keyboard_state(f, chunks[2]),
            2 => self.draw_key_events(f, chunks[2]),
            3 => self.draw_logs(f, chunks[2]),
            _ => {}
        }

        // Draw status bar
        self.draw_status_bar(f, chunks[3]);
    }

    fn draw_title<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let title = Paragraph::new("🎹 ZMK Studio Monitor")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(title, area);
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let titles = vec!["[1] Overview", "[2] Keyboard", "[3] Key Events", "[4] Logs"];

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.selected_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(tabs, area);
    }

    fn draw_overview<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let monitor = self.monitor.blocking_lock();
        let state = monitor.get_state();
        let stats = monitor.get_stats();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),  // Device info
                Constraint::Length(6),  // Battery & Connection
                Constraint::Min(0),     // Stats
            ])
            .split(area);

        // Device info
        if let Some(ref info) = state.device_info {
            let device_text = vec![
                Line::from(vec![
                    Span::styled("Device: ", Style::default().fg(Color::Gray)),
                    Span::styled(&info.name, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("Firmware: ", Style::default().fg(Color::Gray)),
                    Span::styled(&info.firmware, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Layout: ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{} ({} keys)", info.layout, info.key_count), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Layers: ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{}", info.layer_count), Style::default().fg(Color::White)),
                ]),
            ];

            let device_info = Paragraph::new(device_text)
                .block(Block::default().borders(Borders::ALL).title("Device Info"));

            f.render_widget(device_info, chunks[0]);
        }

        // Battery & Connection
        let mut status_lines = vec![];

        if let Some(ref battery) = state.battery {
            let battery_color = if battery.level > 50 {
                Color::Green
            } else if battery.level > 20 {
                Color::Yellow
            } else {
                Color::Red
            };

            status_lines.push(Line::from(vec![
                Span::styled("Battery: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{}% ({:.2}V) {}", battery.level, battery.voltage as f32 / 1000.0,
                        if battery.charging { "⚡" } else { "" }),
                    Style::default().fg(battery_color),
                ),
            ]));
        }

        if let Some(ref conn) = state.connection {
            let conn_type = match conn.connection_type {
                ConnectionType::Usb => "USB",
                ConnectionType::Bluetooth => "Bluetooth",
            };

            let mut conn_text = format!("{}", conn_type);
            if let Some(strength) = conn.signal_strength {
                conn_text.push_str(&format!(" ({} dBm)", strength));
            }

            status_lines.push(Line::from(vec![
                Span::styled("Connection: ", Style::default().fg(Color::Gray)),
                Span::styled(conn_text, Style::default().fg(if conn.connected { Color::Green } else { Color::Red })),
            ]));
        }

        if let Some(ref layer_state) = state.layer_state {
            let layer_name = layer_state.layer_names.get(layer_state.active_layer as usize)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");

            status_lines.push(Line::from(vec![
                Span::styled("Active Layer: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} ({})", layer_name, layer_state.active_layer),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        let status_widget = Paragraph::new(status_lines)
            .block(Block::default().borders(Borders::ALL).title("Status"));

        f.render_widget(status_widget, chunks[1]);

        // Stats
        let stats_text = vec![
            Line::from(vec![
                Span::styled("Session Duration: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{}m {}s", stats.session_duration.as_secs() / 60, stats.session_duration.as_secs() % 60),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("Total Key Presses: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", stats.total_key_presses), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Average WPM: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:.1}", stats.average_wpm), Style::default().fg(Color::Green)),
            ]),
        ];

        let stats_widget = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Statistics"));

        f.render_widget(stats_widget, chunks[2]);
    }

    fn draw_keyboard_state<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let monitor = self.monitor.blocking_lock();
        let state = monitor.get_state();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Layer info
                Constraint::Min(0),     // Layer list
            ])
            .split(area);

        // Active layer details
        if let Some(ref layer_state) = state.layer_state {
            let layer_name = layer_state.layer_names.get(layer_state.active_layer as usize)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");

            let layer_text = vec![
                Line::from(vec![
                    Span::styled("Current Layer: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("{} (Layer {})", layer_name, layer_state.active_layer),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled("Layer represents your current keymap state.", Style::default().fg(Color::DarkGray))),
                Line::from(Span::styled("Different layers provide access to different key functions.", Style::default().fg(Color::DarkGray))),
            ];

            let layer_widget = Paragraph::new(layer_text)
                .block(Block::default().borders(Borders::ALL).title("Active Layer"));

            f.render_widget(layer_widget, chunks[0]);

            // List all layers
            let items: Vec<ListItem> = layer_state.layer_names.iter().enumerate().map(|(idx, name)| {
                let is_active = idx == layer_state.active_layer as usize;
                let style = if is_active {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_active { "➤ " } else { "  " };

                ListItem::new(format!("{}{}: {}", prefix, idx, name)).style(style)
            }).collect();

            let layer_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("All Layers"));

            f.render_widget(layer_list, chunks[1]);
        }
    }

    fn draw_key_events<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let monitor = self.monitor.blocking_lock();
        let events = monitor.get_key_events();

        let items: Vec<ListItem> = events.iter().rev().skip(self.scroll_offset).take(area.height as usize - 2).map(|event| {
            let timestamp = chrono::DateTime::from_timestamp_millis(event.timestamp as i64)
                .map(|dt| dt.format("%H:%M:%S%.3f").to_string())
                .unwrap_or_else(|| "??:??:??".to_string());

            let text = format!(
                "[{}] Layer {} Pos {:2} -> {}",
                timestamp, event.layer, event.position, event.key_code
            );

            ListItem::new(text).style(Style::default().fg(Color::Green))
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(format!("Key Events (Use ↑/↓ to scroll) - {} events", events.len())));

        f.render_widget(list, area);
    }

    fn draw_logs<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let monitor = self.monitor.blocking_lock();
        let logs = monitor.get_log_messages();

        let items: Vec<ListItem> = logs.iter().rev().skip(self.scroll_offset).take(area.height as usize - 2).map(|log| {
            let timestamp = chrono::DateTime::from_timestamp_millis(log.timestamp as i64)
                .map(|dt| dt.format("%H:%M:%S%.3f").to_string())
                .unwrap_or_else(|| "??:??:??".to_string());

            let level_str = match log.level {
                LogLevel::Error => "ERROR",
                LogLevel::Warning => "WARN ",
                LogLevel::Info => "INFO ",
                LogLevel::Debug => "DEBUG",
            };

            let level_color = match log.level {
                LogLevel::Error => Color::Red,
                LogLevel::Warning => Color::Yellow,
                LogLevel::Info => Color::Green,
                LogLevel::Debug => Color::Gray,
            };

            let text = vec![
                Span::styled(format!("[{}] ", timestamp), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} ", level_str), Style::default().fg(level_color).add_modifier(Modifier::BOLD)),
                Span::styled(&log.message, Style::default().fg(Color::White)),
            ];

            ListItem::new(Line::from(text))
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(format!("Debug Logs (Use ↑/↓ to scroll) - {} messages", logs.len())));

        f.render_widget(list, area);
    }

    fn draw_status_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let monitor = self.monitor.blocking_lock();
        let state = monitor.get_state();

        let status_text = if state.connected {
            format!("Connected | Last update: {} | Press 'q' to quit, 1-4 to switch tabs",
                state.last_update.format("%H:%M:%S"))
        } else {
            "Disconnected | Press 'q' to quit".to_string()
        };

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(status, area);
    }
}
