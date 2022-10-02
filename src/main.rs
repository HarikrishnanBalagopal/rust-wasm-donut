fn main() {
    let mut buf = hello_rust::constants::new_buffer();
    hello_rust::render::initialize_buffer(&mut buf);
    let mut t = 100.1;
    loop {
        hello_rust::render::step(&mut buf, t);
        hello_rust::render::draw(&buf);
        t += 0.01;
    }
}
