use limage::*;
use mvideo::*;

fn main() {
    let mut img = Limage::new(512, 512).with_color([0; 3]);
    let mut vid = Mvideo::new(512, 512);
    for i in 0..256 {
        img.draw_circle((256, 256), i, [i as u8, i as u8 + 32, i as u8 + 64]);
        vid.push_frame(&img.as_rgb_buf());
    }

    vid.save("lol");
}