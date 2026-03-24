mod pal;
fn main() {
    pal::connect()
        .expect("We either had a memory error or failed to connect")
        .run()
        .expect("Event loop error");
}
