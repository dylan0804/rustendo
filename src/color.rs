use sdl2::pixels::Color;

pub fn get_rgb_color(byte: u8) -> (u8, u8, u8) {
    let color = match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    };
    color.rgb()
}
