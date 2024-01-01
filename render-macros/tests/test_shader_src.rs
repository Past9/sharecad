use render_macros::shader_src;

#[test]
fn test_shader_src() {
    let src = shader_src!("../tests/test_shader.wgsl");
    assert_eq!(
        "fn second_dependency() { }\nfn first_dependency() { }\nfn main() { }",
        src.trim()
    );
}
