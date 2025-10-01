#[derive(Clone, Copy)]
pub struct AvatarOptions {
    pub width: u32,
    pub height: u32,
    pub render_scale: u32, // 1 = off, 2/4 = supersample
    pub font_scale: f32,
    pub text_y_bias: f32, // negative lifts the letter up
    pub deterministic_color: bool,
    pub sat: f32,
    pub light: f32,
}
