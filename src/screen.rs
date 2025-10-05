use crate::{color, cpu::CPU, mem::Mem};

pub fn should_update_screen(cpu: &CPU, frame: &mut [u8]) -> bool {
    let mut should_update = false;
    let mut frame_idx = 0;

    for addr in 0x0200..0x0600 {
        let color_byte = cpu.mem_read(addr as u16);
        let (r, g, b) = color::get_rgb_color(color_byte);

        // only update the screen if the index changes aka the color
        if frame[frame_idx] != r || frame[frame_idx + 1] != g || frame[frame_idx + 2] != b {
            frame[frame_idx] = r;
            frame[frame_idx + 1] = g;
            frame[frame_idx + 2] = b;
            should_update = true
        }
        frame_idx += 3;
    }
    should_update
}
