use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::game::{Game, NoteState, Screen, APPROACH_TIME};

// ── 캐릭터 스프라이트 (리듬천국의 반응하는 캐릭터 오마주) ──
const IDLE_A: [&str; 3] = ["   ○   ", "  /|\\  ", "  / \\  "];
const IDLE_B: [&str; 3] = ["   ○ ♪ ", "  \\|/  ", "  / \\  "];
const POSE_PERFECT: [&str; 3] = [" \\ ○ / ", "   |   ", "  / \\  "];
const POSE_GOOD: [&str; 3] = ["   ○ / ", "  /|   ", "  / \\  "];
const POSE_MISS: [&str; 3] = ["   ✗   ", "  /|\\  ", "  ∧ ∧  "];

/// 콤보 피버 모드 발동 기준
const FEVER_COMBO: u32 = 10;

pub fn draw(frame: &mut Frame, game: &Game) {
    match game.screen {
        Screen::Playing => draw_playing(frame, game),
        Screen::Result => draw_result(frame, game),
    }
}

/// 현재 진행 중인 섹션 (0=정박, 1=엇박, 2=8비트) → 테마 컬러
fn section_color(game: &Game, now: f64) -> Color {
    let beat = game.beat_len();
    let bars = ((now - game.lead_in) / (beat * 4.0)).floor();
    match (bars / 4.0) as i64 {
        i64::MIN..=0 => Color::Cyan,
        1 => Color::Magenta,
        _ => Color::LightRed,
    }
}

fn draw_playing(frame: &mut Frame, game: &Game) {
    let now = game.now();
    let fever = game.combo >= FEVER_COMBO;
    let theme = section_color(game, now);

    // ── 미스 직후 0.25초간 화면 흔들기 ──
    let mut area = frame.size();
    if let Some(("MISS", at, _)) = game.last_judgment {
        if now - at < 0.25 && ((now * 25.0) as u64) % 2 == 0 {
            area.x += 1;
            area.width = area.width.saturating_sub(1);
        }
    }

    let layout = Layout::vertical([
        Constraint::Length(3), // 헤더
        Constraint::Length(3), // 레인
        Constraint::Length(5), // 무대 (캐릭터 + 판정)
        Constraint::Length(2), // 비트 펄스 / 카운트다운
        Constraint::Min(1),    // 조작법
    ])
    .split(area);

    // ── 헤더 ──────────────────────────────────────
    let title = if fever {
        Span::styled(" ★ FEVER!! ★ ", Style::new().fg(Color::Yellow).bold())
    } else {
        Span::styled(" 터미널 리듬천국 ", Style::new().fg(theme).bold())
    };
    let header = Line::from(vec![
        Span::styled(" SCORE ", Style::new().dim()),
        Span::styled(format!("{:>6}", game.score), Style::new().bold()),
        Span::raw("   "),
        Span::styled(" COMBO ", Style::new().dim()),
        Span::styled(
            format!("{:>3}", game.combo),
            Style::new().bold().fg(if fever { Color::Yellow } else { Color::White }),
        ),
        Span::raw(if fever { " ♪♫♪" } else { "" }),
        Span::raw("   "),
        Span::styled(format!(" {} BPM ", game.bpm as u32), Style::new().dim()),
    ]);
    frame.render_widget(
        Paragraph::new(header).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(theme))
                .title(title),
        ),
        layout[0],
    );

    // ── 노트 레인 ─────────────────────────────────
    let lane_area = layout[1];
    let inner_w = lane_area.width.saturating_sub(2) as usize;
    let hit_col: usize = 6;
    let beat = game.beat_len();
    let phase = (now / beat).fract();

    let mut row: Vec<Span> = vec![Span::raw(" "); inner_w];
    if hit_col < inner_w {
        // 판정선이 비트에 맞춰 두근거림
        let marker = if phase < 0.12 { "█" } else { "▌" };
        row[hit_col] = Span::styled(marker, Style::new().fg(theme).bold());
    }
    for (i, note) in game.notes.iter().enumerate() {
        if note.state != NoteState::Waiting {
            continue;
        }
        let progress = (note.time - now) / APPROACH_TIME;
        if !(0.0..=1.0).contains(&progress) {
            continue;
        }
        let travel = (inner_w - 1 - hit_col) as f64;
        let col = hit_col + (progress * travel).round() as usize;
        if col < inner_w {
            // 피버 중엔 노트가 무지개색으로 순환
            let color = if fever {
                [Color::Red, Color::Yellow, Color::Green, Color::Cyan, Color::Magenta]
                    [(i + (now * 8.0) as usize) % 5]
            } else if progress < 0.15 {
                Color::Yellow
            } else {
                Color::White
            };
            let ch = if fever { "◆" } else { "●" };
            row[col] = Span::styled(ch, Style::new().fg(color).bold());
        }
    }
    frame.render_widget(
        Paragraph::new(Line::from(row)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(theme)),
        ),
        lane_area,
    );

    // ── 무대: 캐릭터 + 판정 텍스트 ────────────────
    let stage = layout[2];
    let cols = Layout::horizontal([Constraint::Length(12), Constraint::Min(10)]).split(stage);

    // 캐릭터: 최근 판정에 따라 포즈, 평소엔 비트에 맞춰 들썩
    let (sprite, sprite_color) = match game.last_judgment {
        Some((text, at, _)) if now - at < 0.4 => match text {
            "PERFECT!!" => (POSE_PERFECT, Color::Yellow),
            "GOOD" => (POSE_GOOD, Color::Green),
            _ => (POSE_MISS, Color::Red),
        },
        _ => {
            if phase < 0.5 {
                (IDLE_A, Color::White)
            } else {
                (IDLE_B, Color::White)
            }
        }
    };
    let mut char_lines: Vec<Line> = vec![Line::raw("")];
    char_lines.extend(
        sprite
            .iter()
            .map(|s| Line::from(Span::styled(*s, Style::new().fg(sprite_color).bold()))),
    );
    frame.render_widget(
        Paragraph::new(char_lines).alignment(Alignment::Center),
        cols[0],
    );

    // 판정 텍스트: 퍼펙트는 반짝이 장식 + 오차 ms 표시
    let judgment_lines: Vec<Line> = match game.last_judgment {
        Some((text, at, ms)) if now - at < 0.5 => {
            let color = match text {
                "PERFECT!!" => Color::Yellow,
                "GOOD" => Color::Green,
                _ => Color::Red,
            };
            let deco = match text {
                "PERFECT!!" => {
                    if ((now * 10.0) as u64) % 2 == 0 { "✧ ✦ ✧" } else { "✦ ✧ ✦" }
                }
                "GOOD" => "♪",
                _ => "…",
            };
            let detail = match ms {
                Some(ms) if text != "MISS" => {
                    format!("{}{:.0}ms", if ms >= 0.0 { "빠름 " } else { "늦음 " }, ms.abs())
                }
                _ => String::new(),
            };
            vec![
                Line::raw(""),
                Line::from(Span::styled(deco, Style::new().fg(color))),
                Line::from(Span::styled(
                    format!("  {}  ", text),
                    Style::new().fg(Color::Black).bg(color).bold(),
                )),
                Line::from(Span::styled(detail, Style::new().dim())),
            ]
        }
        _ => vec![],
    };
    frame.render_widget(
        Paragraph::new(judgment_lines).alignment(Alignment::Center),
        cols[1],
    );

    // ── 비트 펄스 + 카운트다운 ────────────────────
    let beat_idx = (now / beat) as usize % 4;
    let mut pulse: Vec<Span> = Vec::new();
    for i in 0..4 {
        let on = i == beat_idx && phase < 0.25;
        pulse.push(Span::styled(
            if on { "◉ " } else { "○ " },
            Style::new().fg(if on { theme } else { Color::DarkGray }),
        ));
    }
    let countdown = if now < game.lead_in {
        let remaining = ((game.lead_in - now) / beat).ceil() as i64;
        if remaining > 1 {
            format!("  {}", remaining - 1)
        } else {
            "  GO!".to_string()
        }
    } else {
        String::new()
    };
    pulse.push(Span::styled(countdown, Style::new().fg(Color::Yellow).bold()));
    frame.render_widget(
        Paragraph::new(Line::from(pulse)).alignment(Alignment::Center),
        layout[3],
    );

    // ── 조작법 ────────────────────────────────────
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            " [SPACE/J/F] 판정선에서 입력  [Q] 종료 ",
            Style::new().dim(),
        ))),
        layout[4],
    );
}

fn draw_result(frame: &mut Frame, game: &Game) {
    let (rank_ko, rank_en) = game.rank();
    let (rank_color, deco) = match rank_en {
        "HIGH LEVEL" => (Color::Yellow, "✧ ✦ ★ ✦ ✧"),
        "OK" => (Color::Green, "♪ ♫ ♪"),
        _ => (Color::Red, "· · ·"),
    };

    let lines = vec![
        Line::raw(""),
        Line::from(Span::styled(deco, Style::new().fg(rank_color))),
        Line::from(Span::styled(
            format!("  {}  ", rank_ko),
            Style::new().fg(Color::Black).bg(rank_color).bold(),
        )),
        Line::from(Span::styled(rank_en, Style::new().fg(rank_color))),
        Line::from(Span::styled(deco, Style::new().fg(rank_color))),
        Line::raw(""),
        Line::from(format!("SCORE      {:>6}", game.score)),
        Line::from(format!("정확도     {:>5.1}%", game.accuracy() * 100.0)),
        Line::from(format!("최대 콤보  {:>6}", game.max_combo)),
        Line::raw(""),
        Line::from(vec![
            Span::styled("PERFECT ", Style::new().fg(Color::Yellow)),
            Span::raw(format!("{:>3}   ", game.perfects)),
            Span::styled("GOOD ", Style::new().fg(Color::Green)),
            Span::raw(format!("{:>3}   ", game.goods)),
            Span::styled("MISS ", Style::new().fg(Color::Red)),
            Span::raw(format!("{:>3}", game.misses)),
        ]),
        Line::raw(""),
        Line::from(Span::styled("[R] 다시하기   [Q] 종료", Style::new().dim())),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(rank_color))
                    .title(" RESULT "),
            ),
        frame.size(),
    );
}
