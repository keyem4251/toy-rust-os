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
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// 画面への出力を行う
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // 改行の場合何も出力しない
            byte => { // 改行以外の場合バイトを出力する

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
                self.buffer.chars[row][column] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                // 現在の列の位置を進める
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // TODO
    }
}
