fn main() {
    build_deps::rerun_if_changed_paths("*.wgsl").unwrap();
}
