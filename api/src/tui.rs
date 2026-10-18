use std::{
    io::{self, IsTerminal, Write},
    sync::mpsc,
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

const MAX_LINES: usize = 1000;

/// Each formatted tracing event arrives as one write, forwarded to the TUI.
#[derive(Clone)]
struct ChannelWriter(mpsc::Sender<String>);

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = self
            .0
            .send(String::from_utf8_lossy(buf).trim_end().to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Installs the tracing subscriber. With a terminal attached, logs go to a
/// ratatui viewer (q / Ctrl+C to quit); otherwise (docker, CI) plain stdout.
pub fn init() {
    if !io::stdout().is_terminal() {
        tracing_subscriber::fmt::init();
        return;
    }

    let (tx, rx) = mpsc::channel::<String>();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(move || ChannelWriter(tx.clone()))
        .init();

    std::thread::spawn(move || {
        let mut terminal = ratatui::init();
        let mut lines: Vec<String> = Vec::new();

        loop {
            lines.extend(rx.try_iter());
            if lines.len() > MAX_LINES {
                lines.drain(..lines.len() - MAX_LINES);
            }

            let _ = terminal.draw(|frame| {
                let area = frame.area();
                let text: Vec<Line> = lines
                    .iter()
                    .map(|line| {
                        let color = if line.contains(" ERROR ") {
                            Color::Red
                        } else if line.contains(" WARN ") {
                            Color::Yellow
                        } else {
                            Color::Reset
                        };
                        Line::styled(line.as_str(), Style::new().fg(color))
                    })
                    .collect();
                let block = Block::bordered().title(" attendance-service · q to quit ");
                let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
                // stick to the bottom: skip the wrapped rows that don't fit
                let overflow = paragraph
                    .line_count(area.width)
                    .saturating_sub(area.height as usize);
                let paragraph = paragraph.scroll((overflow.min(u16::MAX as usize) as u16, 0));
                frame.render_widget(paragraph, area);
            });

            if event::poll(Duration::from_millis(100)).unwrap_or(false)
                && let Ok(Event::Key(key)) = event::read()
                && (key.code == KeyCode::Char('q')
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)))
            {
                ratatui::restore();
                std::process::exit(0);
            }
        }
    });
}
