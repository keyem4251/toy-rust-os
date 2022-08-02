use volatile::Volatile;
use core::fmt;

// 色を定義
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// 前景と背景の色を指定する
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// 画面上の文字
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// テキストバッファ
// volatileを使用して読み込み、書き込みがrustコンパイラの最適化により取り除かれるのを防ぐ
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// 画面への出力を行う
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    // 1つのASCII文字を書く
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // 改行の場合何も出力しない
            byte => {
                // 改行以外の場合バイトを出力する

                // 現在の行がいっぱいかを確認
                // いっぱいの場合は行を折り返す
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // 現在の場所、色を取得
                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;
                let color_code = self.color_code;

                // 現在の場所に新しい文字を書き込む
                self.buffer.chars[row][column].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // 現在の列の位置を進める
                self.column_position += 1;
            }
        }
    }

    // 文字列全体を出力
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 出力可能なASCIIバイトか改行コード
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 出力可能なASCIIバイトではない
                _ => self.write_byte(0xfe),
            }
        }
    }

    // すべての文字を1行上に持っていき、前の行の最初から始める
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // 現在の行/列の文字
                let character = self.buffer.chars[row][col].read();
                // 1行上に持っていく
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // 0行目はシフトしたら画面から除かれるため削除
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // すべての文字を空白文字で書き換えることでクリア
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.8).unwrap();
}
