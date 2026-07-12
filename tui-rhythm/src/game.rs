use std::time::Instant;

/// 노트가 화면 오른쪽 끝에서 판정선까지 이동하는 시간 (초)
pub const APPROACH_TIME: f64 = 1.8;
pub const PERFECT_WINDOW: f64 = 0.045; // ±45ms
pub const GOOD_WINDOW: f64 = 0.100; // ±100ms
pub const MISS_WINDOW: f64 = 0.150; // 이 범위 밖 입력은 무시, 지나가면 미스

#[derive(Clone, Copy, PartialEq)]
pub enum Judgment {
    Perfect,
    Good,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NoteState {
    Waiting,
    Hit(Judgment),
    Missed,
}

pub struct Note {
    /// 게임 시작 기준, 이 노트를 쳐야 하는 시각 (초)
    pub time: f64,
    pub state: NoteState,
}

#[derive(PartialEq)]
pub enum Screen {
    Playing,
    Result,
}

pub struct Game {
    start: Instant,
    pub bpm: f64,
    pub lead_in: f64, // 카운트다운 길이 (초)
    pub notes: Vec<Note>,
    pub score: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub perfects: u32,
    pub goods: u32,
    pub misses: u32,
    /// (표시할 텍스트, 판정이 발생한 시각, 오차 ms — 미스면 None)
    pub last_judgment: Option<(&'static str, f64, Option<f64>)>,
    pub screen: Screen,
}

impl Game {
    pub fn new(bpm: f64) -> Self {
        let beat = 60.0 / bpm;
        let lead_in = beat * 4.0; // 4박 카운트다운

        // 리듬천국 스타일: 마디(4박)별 패턴을 섹션으로 구성
        // 숫자는 마디 내 박자 위치 (0 = 첫 박)
        let sections: [(&[f64], usize); 3] = [
            (&[0.0, 1.0, 2.0, 3.0], 4),           // 워밍업: 정박 4비트
            (&[0.0, 1.5, 2.0, 3.5], 4),           // 당김음 (엇박)
            (&[0.0, 0.5, 1.0, 2.0, 2.5, 3.0], 4), // 8비트 섞기
        ];

        let mut notes = Vec::new();
        let mut bar_start = lead_in;
        for (pattern, bars) in sections {
            for _ in 0..bars {
                for &b in pattern {
                    notes.push(Note {
                        time: bar_start + b * beat,
                        state: NoteState::Waiting,
                    });
                }
                bar_start += 4.0 * beat;
            }
        }

        Self {
            start: Instant::now(),
            bpm,
            lead_in,
            notes,
            score: 0,
            combo: 0,
            max_combo: 0,
            perfects: 0,
            goods: 0,
            misses: 0,
            last_judgment: None,
            screen: Screen::Playing,
        }
    }

    pub fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn beat_len(&self) -> f64 {
        60.0 / self.bpm
    }

    /// 매 프레임 호출: 놓친 노트 처리 + 종료 판정
    pub fn update(&mut self) {
        let now = self.now();
        for note in &mut self.notes {
            if note.state == NoteState::Waiting && now > note.time + MISS_WINDOW {
                note.state = NoteState::Missed;
                self.misses += 1;
                self.combo = 0;
                self.last_judgment = Some(("MISS", now, None));
            }
        }
        if let Some(last) = self.notes.last() {
            if now > last.time + 1.5 {
                self.screen = Screen::Result;
            }
        }
    }

    /// 키 입력 처리: 가장 가까운 대기 노트를 찾아 판정
    pub fn hit(&mut self) {
        let now = self.now();
        // 판정 범위 안에서 시간 차가 가장 작은 노트 탐색
        let target = self
            .notes
            .iter_mut()
            .filter(|n| n.state == NoteState::Waiting)
            .map(|n| (n.time - now, n))
            .filter(|(diff, _)| diff.abs() <= MISS_WINDOW)
            .min_by(|a, b| a.0.abs().partial_cmp(&b.0.abs()).unwrap());

        let Some((diff, note)) = target else {
            return; // 노트 없는 곳에서 누른 헛스윙은 무시 (관대한 룰)
        };

        let abs = diff.abs();
        let ms = diff * 1000.0;
        if abs <= PERFECT_WINDOW {
            note.state = NoteState::Hit(Judgment::Perfect);
            self.perfects += 1;
            self.score += 100;
            self.combo += 1;
            self.last_judgment = Some(("PERFECT!!", now, Some(ms)));
        } else if abs <= GOOD_WINDOW {
            note.state = NoteState::Hit(Judgment::Good);
            self.goods += 1;
            self.score += 50;
            self.combo += 1;
            self.last_judgment = Some(("GOOD", now, Some(ms)));
        } else {
            note.state = NoteState::Missed;
            self.misses += 1;
            self.combo = 0;
            self.last_judgment = Some(("MISS", now, Some(ms)));
        }
        self.max_combo = self.max_combo.max(self.combo);
    }

    pub fn accuracy(&self) -> f64 {
        let total = self.notes.len() as f64;
        if total == 0.0 {
            return 0.0;
        }
        self.score as f64 / (total * 100.0)
    }

    /// 리듬천국식 랭크
    pub fn rank(&self) -> (&'static str, &'static str) {
        let acc = self.accuracy();
        if acc >= 0.9 {
            ("하이레벨!", "HIGH LEVEL")
        } else if acc >= 0.6 {
            ("평범", "OK")
        } else {
            ("다시...", "TRY AGAIN")
        }
    }
}
