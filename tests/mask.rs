use rlottie_core::loader::json;

#[test]
fn render_alpha_mask() {
    let data = include_bytes!("data/mask.json");
    let comp = json::from_slice(data).unwrap();
    let mut buf = vec![0u8; 10 * 10 * 4];
    comp.render_sync(0, &mut buf, 10, 10, 10 * 4);
    let inside = 5 * 10 * 4 + 5 * 4;
    assert_eq!(&buf[inside..inside + 4], &[255, 0, 0, 255]);
    let outside = 1 * 10 * 4 + 1 * 4;
    assert_eq!(&buf[outside..outside + 4], &[0, 0, 0, 0]);
}
