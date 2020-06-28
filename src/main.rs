use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Write},
    process::abort,
    thread::sleep,
    time,
};
use termion::{
    async_stdin, color, cursor, event::Key, input::TermRead, raw::IntoRawMode,
    screen::AlternateScreen, style,
};

const N: usize = 8;

struct Float {
    val: u64,
    x: usize,
    y: usize,
}
impl Float {
    fn new(area: &[[u64; N]; N]) -> Float {
        let mut rng = thread_rng();

        if area.iter().all(|&x| x[0] != 0) {
            abort()
        }

        let mut x = rng.gen::<usize>() % N;
        while area[x][0] != 0 {
            x = rng.gen::<usize>() % N;
        }

        Float {
            val: 2u64.pow(rng.gen::<u32>() % 6 + 1),
            x: x,
            y: 0,
        }
    }
}

trait Area {
    fn set(&mut self, float: &Float);
}
impl Area for [[u64; N]; N] {
    fn set(&mut self, float: &Float) {
        self[float.x][float.y] = float.val;
        if float.y + 1 < N && self[float.x][float.y + 1] == float.val {
            self[float.x][float.y] = 0;
            self.set(&Float {
                val: float.val * 2,
                x: float.x,
                y: float.y + 1,
            });
        } else if float.y > 0 && self[float.x][float.y - 1] == float.val {
            self[float.x][float.y] = 0;
            self.set(&Float {
                val: float.val * 2,
                x: float.x,
                y: float.y - 1,
            });
        } else if float.x + 1 < N && self[float.x + 1][float.y] == float.val {
            self[float.x][float.y] = 0;
            self.set(&Float {
                val: float.val * 2,
                x: float.x + 1,
                y: float.y,
            });
        } else if float.x > 0 && self[float.x - 1][float.y] == float.val {
            self[float.x][float.y] = 0;
            self.set(&Float {
                val: float.val * 2,
                x: float.x - 1,
                y: float.y,
            });
        }
    }
}

type Writer = termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>;

fn draw_number(stdout: &mut Writer, val: u64) {
    match val {
        0 => write!(stdout, "{}", color::Fg(color::White)).unwrap(),
        2..=4 => write!(stdout, "{}", color::Fg(color::Cyan)).unwrap(),
        8..=16 => write!(stdout, "{}", color::Fg(color::Blue)).unwrap(),
        32..=64 => write!(stdout, "{}", color::Fg(color::Yellow)).unwrap(),
        128..=256 => write!(stdout, "{}", color::Fg(color::Magenta)).unwrap(),
        _ => write!(stdout, "{}", color::Fg(color::Red)).unwrap(),
    }
    write!(stdout, "{}{:^5}{}", style::Invert, val, style::Reset).unwrap();
}

fn draw(stdout: &mut Writer, area: &[[u64; N]; N], float: &Float) {
    writeln!(
        stdout,
        "{}{}SCORE: {:06}{}\r",
        cursor::Goto(1, 1),
        color::Fg(color::Cyan),
        area.iter()
            .flat_map(|c| c)
            .collect::<Vec<&u64>>()
            .iter()
            .fold(0, |mut sum, &x| {
                sum += x;
                sum
            }),
        style::Reset
    )
    .unwrap();
    for y in 0..N {
        for x in 0..N {
            if float.x == x && float.y == y {
                draw_number(stdout, float.val);
            } else {
                draw_number(stdout, area[x][y]);
            }
        }
        writeln!(stdout, "\r").unwrap();
    }
}

fn main() {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut stdin = async_stdin().keys();

    let mut area = [[0u64; N]; N];
    let mut float = Float::new(&area);
    let mut cnt = 0u64;

    loop {
        draw(&mut stdout, &area, &float);

        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') => return,
                Key::Char('j') | Key::Down => {
                    if float.y < N - 1 && area[float.x][float.y + 1] == 0 {
                        float.y += 1;
                    }
                }
                Key::Char('h') | Key::Left => {
                    if float.x > 0 && area[float.x - 1][float.y] == 0 {
                        float.x -= 1;
                    }
                }

                Key::Char('l') | Key::Right => {
                    if float.x < N - 1 && area[float.x + 1][float.y] == 0 {
                        float.x += 1;
                    }
                }
                _ => {}
            }
        }

        if cnt % 50 == 0 {
            float.y += 1;
            if float.y == N || area[float.x][float.y] != 0 {
                float.y -= 1;
                area.set(&float);
                float = Float::new(&area);
            }
        }

        cnt += 1;
        sleep(time::Duration::from_millis(10));
    }
}
