mod game;
mod ui;

use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use game::{Game, Screen};

const BPM: f64 = 110.0;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let result = run(&mut terminal);

    // 패닉이 나도 터미널 복구가 되도록 정리 로직은 항상 실행
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut game = Game::new(BPM);

    loop {
        // ~8ms 폴링 ≈ 120fps 상한. 타이밍 판정은 프레임이 아니라
        // Instant 기반이라 프레임 드랍이 판정 정확도에 영향을 주지 않음.
        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                // Windows는 Press/Release 이벤트를 둘 다 보내므로 반드시 필터링
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match (&game.screen, key.code) {
                    (_, KeyCode::Char('q') | KeyCode::Esc) => return Ok(()),
                    (Screen::Playing, KeyCode::Char(' ') | KeyCode::Char('j') | KeyCode::Char('f')) => {
                        game.hit();
                    }
                    (Screen::Result, KeyCode::Char('r')) => {
                        game = Game::new(BPM);
                    }
                    _ => {}
                }
            }
        }

        game.update();
        terminal.draw(|frame| ui::draw(frame, &game))?;
    }
}
