use minimp4::Mp4Muxer;
use openh264::encoder::{Encoder, EncoderConfig};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct Mvideo {
    width: u32,
    height: u32,
    encoder: Encoder,
    buf: Vec<u8>
}
impl Mvideo {
    pub fn new(width: u32, height: u32) -> Self {
        let config = EncoderConfig::new(width, height);
        let encoder = Encoder::with_config(config).unwrap();
        let buf = Vec::new();

        Mvideo { width, height, encoder, buf }
    }

    pub fn push_frame(&mut self, frame: &[u8]) {
        let mut yuv = openh264::formats::YUVBuffer::new(self.width as usize, self.height as usize);
        yuv.read_rgb(frame);

        // Encode YUV into H.264.
        let bitstream = self.encoder.encode(&yuv).unwrap();
        bitstream.write_vec(&mut self.buf);
    }

    pub fn save(&self, filename: &str) {
        let mut video_buffer = Cursor::new(Vec::new());
        let mut mp4muxer = Mp4Muxer::new(&mut video_buffer);
        mp4muxer.init_video(512, 512, false, "Moving circle.");
        mp4muxer.write_video(&self.buf);
        mp4muxer.close();

        // Some shenanigans to get the raw bytes for the video.
        video_buffer.seek(SeekFrom::Start(0)).unwrap();
        let mut video_bytes = Vec::new();
        video_buffer.read_to_end(&mut video_bytes).unwrap();

        std::fs::write("circle.mp4", &video_bytes).unwrap();
    }
}