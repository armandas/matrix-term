use crossterm::style::Stylize;
use crossterm::{cursor, event, style, terminal, QueueableCommand};
use rand::rngs::ThreadRng;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
use std::io::Write;
use std::time::Duration;

const CHARS: &[&str; 294] = &[
    "ァ", "ア", "ィ", "イ", "ゥ", "ウ", "ェ", "エ", "ォ", "オ", "カ", "ガ", "キ", "ギ", "ク", "グ",
    "ケ", "ゲ", "コ", "ゴ", "サ", "ザ", "シ", "ジ", "ス", "ズ", "セ", "ゼ", "ソ", "ゾ", "タ", "ダ",
    "チ", "ヂ", "ッ", "ツ", "ヅ", "テ", "デ", "ト", "ド", "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ", "バ",
    "パ", "ヒ", "ビ", "ピ", "フ", "ブ", "プ", "ヘ", "ベ", "ペ", "ホ", "ボ", "ポ", "マ", "ミ", "ム",
    "メ", "モ", "ャ", "ヤ", "ュ", "ユ", "ョ", "ヨ", "ラ", "リ", "ル", "レ", "ロ", "ヮ", "ワ", "ヰ",
    "ヱ", "ヲ", "ン", "ヴ", "ヵ", "ヶ", "ぁ", "あ", "ぃ", "い", "ぅ", "う", "ぇ", "え", "ぉ", "お",
    "か", "が", "き", "ぎ", "く", "ぐ", "け", "げ", "こ", "ご", "さ", "ざ", "し", "じ", "す", "ず",
    "せ", "ぜ", "そ", "ぞ", "た", "だ", "ち", "ぢ", "っ", "つ", "づ", "て", "で", "と", "ど", "な",
    "に", "ぬ", "ね", "の", "は", "ば", "ぱ", "ひ", "び", "ぴ", "ふ", "ぶ", "ぷ", "へ", "べ", "ぺ",
    "ほ", "ぼ", "ぽ", "ま", "み", "む", "め", "も", "ゃ", "や", "ゅ", "ゆ", "ょ", "よ", "ら", "り",
    "る", "れ", "ろ", "ゎ", "わ", "ゐ", "ゑ", "を", "ん", "ゔ", "ゕ", "ゖ", "ゝ", "ゞ", "ㄶ", "ㄷ",
    "ㄹ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅁ", "ㅂ", "ㅄ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ",
    "ㅋ", "ㅌ", "ㅍ", "ㅎ", "A ", "Ą ", "B ", "C ", "Č ", "D ", "E ", "Ę ", "Ė ", "F ", "G ", "H ",
    "I ", "Į ", "Y ", "J ", "K ", "L ", "M ", "N ", "O ", "P ", "R ", "S ", "Š ", "T ", "U ", "Ų ",
    "Ū ", "V ", "Z ", "Ž ", "a ", "ą ", "b ", "c ", "č ", "d ", "e ", "ę ", "ė ", "f ", "g ", "h ",
    "i ", "į ", "y ", "j ", "k ", "l ", "m ", "n ", "o ", "p ", "r ", "s ", "š ", "t ", "u ", "ų ",
    "ū ", "v ", "z ", "ž ", "0 ", "1 ", "2 ", "3 ", "4 ", "5 ", "6 ", "7 ", "8 ", "9 ", "! ", "@ ",
    "# ", "$ ", "% ", "^ ", "& ", "* ", "( ", ") ", "_ ", "+ ", "{ ", "} ", "[ ", "] ", ": ", "; ",
    "< ", "> ", ", ", ". ", "/ ", "? ",
];

const MAX_AGE: u8 = 20;

#[derive(Clone, Copy, Debug)]
struct Cell {
    pub content: &'static str,
    pub age: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: "  ",
            age: 0,
        }
    }
}

impl Cell {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let content = CHARS.choose(rng).unwrap();
        let age = 0;
        Self { content, age }
    }
}



fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    stdout.queue(terminal::EnterAlternateScreen)?;
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::Hide)?;
    stdout.flush()?;

    let size = crossterm::terminal::size()?;
    let width = size.0 as usize / 2;
    let height = size.1 as usize;
    let mut rng = thread_rng();

    let mut buffer = vec![vec![Option::<Cell>::None; width]; height];

    loop {
        // Age
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = &mut buffer[y][x] {
                    cell.age += if cell.age < MAX_AGE { 1 } else { 0 };
                }
            }
        }

        // Spawn parent
        let spawn_col = buffer[0]
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_none())
            .choose(&mut rng)
            .unwrap_or_else(|| {
                buffer[0]
                    .iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.unwrap().age > 10)
                    .choose(&mut rng)
                    .unwrap()
            })
            .0;
        buffer[0][spawn_col] = Some(Cell::new(&mut rng));

        // Span children
        for y in 1..height {
            for x in 0..width {
                if let Some(cell) = &mut buffer[y - 1][x] {
                    if cell.age == 1 {
                        // println!("{x}.{y}: {cell:?}");
                        buffer[y][x] = Some(Cell::new(&mut rng));
                    }
                }
            }
        }

        // Render
        stdout.queue(cursor::MoveTo(0, 0))?;
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buffer[y][x] {
                    let c = (255.0 * (1.0 - ((MAX_AGE - cell.age) as f32 / MAX_AGE as f32))) as u8;
                    let styled = format!("{}", cell.content).with(style::Color::Rgb { r: c, g: c, b: c });
                    print!("{}", styled);
                } else {
                    print!("  ");
                }
            }
            stdout.queue(cursor::MoveTo(0, y as u16))?;
        }
        stdout.flush()?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            match event::read()? {
                event::Event::Key(event) => {
                    if event.code == event::KeyCode::Char('q') {
                        break;
                    }
                }
                _ => (),
            };
        }
    }

    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.queue(cursor::Show)?;
    stdout.flush()?;

    Ok(())
}
