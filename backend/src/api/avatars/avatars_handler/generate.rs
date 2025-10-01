use anyhow::{anyhow, Context};
use image::{imageops::FilterType, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut, text_size};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rusttype::{point, Font, Scale};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use super::types::AvatarOptions;

pub fn generate_and_store_avatar(user_id: i64, username: &str) -> anyhow::Result<String> {
    let font_path = "assets/fonts/DejaVuSans-Bold.ttf";
    // Ensure the directory exists (once; harmless if it exists)
    std::fs::create_dir_all("/app/uploads/avatars").context("creating /app/uploads/avatars")?;

    // Filesystem path inside the container
    let mut out_path = PathBuf::from("/app/uploads/avatars");
    out_path.push(format!("{user_id}.png"));

    generate_avatar(username, font_path, out_path.to_str().unwrap())?;

    // Public URL for DB (served by static mount)
    let avatar_url = format!("/static/avatars/{user_id}.png");
    Ok(avatar_url)
}

/// Now returns anyhow::Result<()>
fn generate_avatar(name: &str, font_path: &str, out_path: &str) -> anyhow::Result<()> {
    let font = load_font_from_assets(font_path)
        .with_context(|| format!("loading font at '{font_path}'"))?;

    let opts = AvatarOptions {
        width: 512,
        height: 512,
        render_scale: 1, // use 2 or 4 for supersampling + downscale
        font_scale: 0.95,
        text_y_bias: -0.78, // lift slightly for better visual centering
        deterministic_color: true,
        sat: 0.65,
        light: 0.55,
    };

    // Render size (supersampling)
    let rw = opts.width * opts.render_scale;
    let rh = opts.height * opts.render_scale;

    // Transparent canvas
    let mut img: RgbaImage = ImageBuffer::from_pixel(rw, rh, Rgba([0, 0, 0, 0]));

    // Background
    let (r, g, b) = background_color(name, opts);
    let bg = Rgba([r, g, b, 255]);

    // Circle
    let cx = (rw / 2) as i32;
    let cy = (rh / 2) as i32;
    let radius = (rw.min(rh) / 2) as i32;
    draw_filled_circle_mut(&mut img, (cx, cy), radius, bg);

    // Letter + centering
    let text = first_letter(name).to_string();
    let scale = Scale::uniform((rw.min(rh) as f32) * opts.font_scale);
    let (bx, by) = centered_baseline(scale, &font, &text, rw, rh, opts.text_y_bias);
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        bx,
        by,
        scale,
        &font,
        &text,
    );

    // Downscale if needed
    let final_img = if opts.render_scale > 1 {
        image::imageops::resize(&img, opts.width, opts.height, FilterType::Lanczos3)
    } else {
        img
    };

    final_img
        .save(out_path)
        .with_context(|| format!("saving PNG to '{out_path}'"))?;
    Ok(())
}

/// Use anyhow::Result here too, so callers compose cleanly.
fn load_font_from_assets(path: &str) -> anyhow::Result<Font<'static>> {
    let bytes = fs::read(path).with_context(|| format!("reading font file '{path}'"))?;
    Font::try_from_vec(bytes).ok_or_else(|| anyhow!("failed to parse TTF/OTF at '{path}'"))
}

fn centered_baseline(
    scale: Scale,
    font: &Font,
    text: &str,
    img_w: u32,
    img_h: u32,
    y_bias_frac: f32,
) -> (i32, i32) {
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();

    let mut bb_opt: Option<rusttype::Rect<i32>> = None;
    for g in &glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            bb_opt = Some(match bb_opt {
                None => bb,
                Some(acc) => rusttype::Rect {
                    min: point(acc.min.x.min(bb.min.x), acc.min.y.min(bb.min.y)),
                    max: point(acc.max.x.max(bb.max.x), acc.max.y.max(bb.max.y)),
                },
            });
        }
    }

    if let Some(bb) = bb_opt {
        let tw = (bb.max.x - bb.min.x) as i32;
        let th = (bb.max.y - bb.min.y) as i32;

        let x0 = (img_w as i32 - tw) / 2;
        let mut y0 = (img_h as i32 - th) / 2;

        y0 += ((img_h as f32) * y_bias_frac) as i32;

        let baseline_x = x0 - bb.min.x;
        let baseline_y = y0 - bb.min.y;
        (baseline_x, baseline_y)
    } else {
        let (tw, th) = text_size(scale, font, text);
        let x = (img_w as i32 - tw as i32) / 2;
        let mut y = (img_h as i32 - th as i32) / 2;
        y += ((img_h as f32) * y_bias_frac) as i32;
        (x, y)
    }
}

fn first_letter(name: &str) -> char {
    name.trim()
        .chars()
        .next()
        .map(|c| c.to_uppercase().next().unwrap_or(c))
        .unwrap_or('#')
}

fn background_color(name: &str, opts: AvatarOptions) -> (u8, u8, u8) {
    let mut hasher = DefaultHasher::new();
    name.to_lowercase().hash(&mut hasher);
    let h = hasher.finish();

    let mut seed = [0u8; 32];
    for i in 0..4 {
        seed[i * 8..(i + 1) * 8].copy_from_slice(&h.to_le_bytes());
    }

    let mut rng: StdRng = if opts.deterministic_color {
        StdRng::from_seed(seed)
    } else {
        StdRng::from_entropy()
    };

    let hue = rng.gen_range(0.0..360.0);
    hsl_to_rgb(hue, opts.sat, opts.light)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let (r1, g1, b1) = match h_prime {
        hp if (0.0..1.0).contains(&hp) => (c, x, 0.0),
        hp if (1.0..2.0).contains(&hp) => (x, c, 0.0),
        hp if (2.0..3.0).contains(&hp) => (0.0, c, x),
        hp if (3.0..4.0).contains(&hp) => (0.0, x, c),
        hp if (4.0..5.0).contains(&hp) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let to8 = |v: f32| ((v + m).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    (to8(r1), to8(g1), to8(b1))
}
