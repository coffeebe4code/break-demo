use break_main::main_work;

fn main() {
    pollster::block_on(main_work());
}
