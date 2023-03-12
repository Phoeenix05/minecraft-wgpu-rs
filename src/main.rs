mod lib;
use lib::run;

#[allow(dead_code)]
fn main() {
    pollster::block_on(run());
}
