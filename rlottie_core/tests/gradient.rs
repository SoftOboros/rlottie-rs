use rlottie_core::geometry::Path;
use rlottie_core::renderer::cpu::draw_path;
use rlottie_core::types::{Color, GradientStop, LinearGradient, Paint, Vec2};

#[test]
fn linear_gradient_rect() {
    let mut path = Path::new();
    path.move_to(Vec2 { x: 0.0, y: 0.0 });
    path.line_to(Vec2 { x: 8.0, y: 0.0 });
    path.line_to(Vec2 { x: 8.0, y: 8.0 });
    path.line_to(Vec2 { x: 0.0, y: 8.0 });
    path.close();
    let grad = LinearGradient {
        start: Vec2 { x: 0.0, y: 0.0 },
        end: Vec2 { x: 8.0, y: 0.0 },
        stops: vec![
            GradientStop {
                offset: 0.0,
                color: Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            },
            GradientStop {
                offset: 1.0,
                color: Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
            },
        ],
    };
    let mut buf = vec![0u8; 8 * 8 * 4];
    draw_path(&path, Paint::Linear(grad), &mut buf, 8, 8, 8 * 4);
    let left = 4 * 4; // (0,0)
    let right = 7 * 4 + 7 * 8 * 4;
    assert!(buf[left] > buf[right]);
    assert!(buf[right + 2] > buf[left + 2]);
}
