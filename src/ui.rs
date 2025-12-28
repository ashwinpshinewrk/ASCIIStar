use crate::app::App;
use crate::engine::{Point3D, Rng};

use glam::{Mat4, Vec3};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

const RNG_SEED: u32 = 777;

pub fn render(f: &mut Frame, app: &mut App, time: f32) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    let area = chunks[0];

    let mut scene_ponts: Vec<Point3D> = Vec::new();
    let mut rng = Rng::new(RNG_SEED);

    // TRUNK
    for y_step in 0..25 {
        let t = y_step as f32 / 25.0;
        for r_l in 1..6 {
            let r = (1.1 - t * 0.3) * (r_l as f32 / 6.0);
            for i in 0..30 {
                let a = (i as f32 / 35.0) * 6.28;
                scene_ponts.push(Point3D {
                    pos: Vec3::new(a.cos() * r, -6.0 + t * 3.0, a.sin() * r),
                    color: Color::Rgb(95, 55, 30),
                    is_bright: false,
                    is_ornament: false,
                    ornament_id: 0,
                    birth_threshold: t * 0.1,
                    is_star: false,
                    lod_level: 0.0,
                    fixed_char: None,
                });
            }
        }
    }

    // LUSH TREE

    for tier in (0..4).rev() {
        let t_off = (3 - tier) as f32 * 0.15;
        let y_start = 3.0 - (tier as f32 * 1.6);
        let max_r = 1.4 + (tier as f32 * 1.2);

        for s in 0..28 {
            let t = s as f32 / 28.0;
            let y = y_start - (t * 2.8);
            let sr = max_r * t;
            let birth = 0.1 + t_off + (t + 0.1);

            for r_idx in 1..=8 {
                let r = sr * (r_idx as f32 / 8.0);
                let num_p = (r * 50.0).max(15.0) as usize;
                for p in 0..num_p{
                    let a = (p as f32 / 10.0) * 6.28;
                    let is_orn = rng.next_f32() > 0.94;
                    scene_ponts.push(Point3D {
                        pos: Vec3::new(a.cos() * r, y, a.sin() * r),
                        color: Color::Rgb(0, (70 + tier * 40 + s * 2).min(255) as u8, 20),
                        is_bright: is_orn,
                        is_ornament: is_orn,
                        ornament_id: p + s,
                        birth_threshold: birth,
                        is_star: false,
                        lod_level: 0.0,
                        fixed_char: None,
                    });
                }
            }
        }
    }

    let star_center = Vec3::new(0.0, 3.6, 0.0);
    let outer_r = 1.0;
    let inner_r = 0.45;
    let depth = 0.3;
    let mut outer_v = Vec::new();
    for i in 0..5 {
        let a_o = (i as f32 * 72.0).to_radians();
        outer_v.push(Vec3::new(a_o.sin() * outer_r, a_o.cos() * outer_r, 0.0));
    }
    let mut inner_v = Vec::new();

    for i in 0..5 {
        let a_i = (i as f32 * 72.0 + 36.0).to_radians();
        inner_v.push(Vec3::new(a_i.sin() * inner_r, a_i.cos() * inner_r, 0.0));
    }

    let front_c = Vec3::new(0.0, 0.0, depth);
    let back_c = Vec3::new(0.0, 0.0, -depth);

    for i in 0..5 {
        let prev = (i + 4) % 5;
        let faces = [
            (front_c, outer_v[i], inner_v[i], 1.0),
            (front_c, outer_v[i], inner_v[prev], 0.75),
            (back_c, outer_v[i], inner_v[i], 0.5),
        ];

        for (v1, v2, v3, shade) in faces {
            let density = if app.is_zooming { 900 } else { 180 };

            for _ in 0..density {
                let mut a = rng.next_f32();
                let mut b = rng.next_f32();

                if a + b > 1.0 {
                    a = 1.0 - a;
                    b = 1.0 - b;
                }

                let p = v1 + a * (v2 - v1) + b * (v3 - v1);
                scene_ponts.push(Point3D {
                    pos: star_center + p,
                    color: Color::Rgb((255.0 * shade) as u8, (210.0 * shade) as u8, 0),
                    is_bright: shade > 0.8,
                    is_ornament: false,
                    birth_threshold: 0.85,
                    ornament_id: 0,
                    is_star: true,
                    lod_level: if density > 200 { 0.4 } else { 0.0 },
                    fixed_char: None,
                });
            }
        }
    }

    //ARC
    if app.zoom_progress > 0.1 {
        let arc_text = "MERRY CHRISTMAS";
        let blink_phase = (time * 6.0) as usize % 2 == 0;
        for (i, c) in arc_text.chars().enumerate() {
            let angle =
                std::f32::consts::PI * (0.75 - (i as f32 / (arc_text.len() as f32 - 1.0)) * 0.5);

            let r = 1.3;
            let color = if blink_phase ^ (i % 2 == 0) {
                Color::Rgb(255, 50, 50)
            } else {
                Color::Rgb(50, 255, 50)
            };
            scene_ponts.push(Point3D {
                pos: star_center + Vec3::new(angle.cos() * r, angle.sin() * r, 0.0),
                is_bright: true,
                is_ornament: false,
                ornament_id: i,
                birth_threshold: 0.0,
                is_star: false,
                fixed_char: Some(c),
                lod_level: 0.5,
                color,
            });
        }
    }

    let b_scale = area.height as f32 / 14.0;
    let current_scale = b_scale + (area.height as f32 / 2.6 - b_scale) * app.zoom_progress;
    let current_y_off = -3.2 * app.zoom_progress;
    let rotation_mat = Mat4::from_euler(glam::EulerRot::XYZ, app.angle_x, app.angle_y, 0.0);
    let chars: Vec<char> = app.input.chars().collect();

    let mut projected: Vec<(f32, f32, f32, Color, char, bool, bool, bool, bool)> = scene_ponts
        .into_iter()
        .filter(|p| p.birth_threshold <= app.anim_progress)
        .filter(|p| p.lod_level <= app.zoom_progress + 0.1)
        .enumerate()
        .map(|(i, p)| {
            let mut p_pos = p.pos;
            p_pos.y += current_y_off;
            let rot_p = rotation_mat.project_point3(p_pos);
            let mut col = p.color;
            let is_greeting = p.fixed_char.is_some();

            if p.is_star && !is_greeting {
                let shm = ((time * 8.0 + (p.pos.x * 15.0)).sin() * 25.0) as i8;

                if let Color::Rgb(r, g, b) = col {
                    col = Color::Rgb(
                        r.saturating_add_signed(shm),
                        g.saturating_add_signed(shm),
                        b.saturating_add_signed(shm / 2),
                    );
                }
            } else if p.is_ornament {
                let phase = (p.ornament_id as f32 + time * 4.0) as usize % 4;
                col = match phase {
                    0 => Color::Rgb(255, 60, 60),
                    1 => Color::Rgb(60, 180, 255),
                    2 => Color::Rgb(255, 215, 0),
                    _ => Color::Rgb(100, 255, 100),
                };
            }
            (
                rot_p.x,
                rot_p.y,
                rot_p.z,
                col,
                p.fixed_char.unwrap_or(chars[i % chars.len()]),
                p.is_bright,
                p.is_star,
                false,
                is_greeting,
            )
        })
        .collect();

    if app.anim_progress >= 1.0 {
        for (i, s) in app.snow.iter().enumerate() {
            let mut sp = s.pos;
            sp.y += current_y_off;
            let rot_p = rotation_mat.project_point3(sp);
            projected.push((
                rot_p.x,
                rot_p.y,
                rot_p.z,
                Color::Rgb(180, 180, 255),
                if i % 2 == 0 { '*' } else { '.' },
                true,
                false,
                true,
                false,
            ));
        }
    }
    projected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    //RENDERRRRR
    for (px, py, pz, color, c, is_br, is_st, is_sn, is_gr) in projected {
        let x = (area.x as f32 + area.width as f32 / 2.0) + (px * current_scale * 4.8);
        let y = (area.y as f32 + area.height as f32 / 2.0) - (py * current_scale);

        if x >= area.left() as f32
            && x < area.right() as f32
            && y >= area.top() as f32
            && y < area.bottom() as f32
        {
            let mut inten = (0.5 + (pz + 6.0) / 12.0).clamp(0.2, 1.4);
            if app.zoom_progress > 0.4 && !is_st && !is_sn && !is_gr {
                inten *= 1.0 - (app.zoom_progress - 0.4) * 1.5;
            }

            let final_color = match color {
                Color::Rgb(r, g, b) => Color::Rgb(
                    (r as f32 * inten.max(0.1)) as u8,
                    (g as f32 * inten.max(0.1)) as u8,
                    (b as f32 * inten.max(0.1)) as u8,
                ),
                _ => color,
            };

            f.buffer_mut()[(x as u16, y as u16)]
                .set_char(c)
                .set_fg(final_color)
                .set_style(Style::default().add_modifier(if is_br {
                    Modifier::BOLD
                } else {
                    Modifier::DIM
                }));

            if is_st && !is_gr && app.zoom_progress > 0.8 && rng.next_f32() > 0.98 {
                let fx = (x as i16 + (rng.next_f32() * 8.0 - 4.0) as i16).max(0) as u16;
                let fy = (y as i16 + (rng.next_f32() * 4.0 - 2.0) as i16).max(0) as u16;

                if fx < area.right() && fy < area.bottom() {
                    f.buffer_mut()[(fx, fy)]
                        .set_char(['⢄', '⣦', '⡀', '⠋'][rng.next_f32() as usize % 4])
                        .set_fg(Color::Rgb(255, 255, 200));
                }
            }
        }
    }

    let foot = Paragraph::new(format!(
        "WORD : {} | s : Zoom Star | Enter : Grow ",
        app.input
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("ASCII CHRISTMAS TREE"),
    );

    f.render_widget(foot, chunks[1]);
}
