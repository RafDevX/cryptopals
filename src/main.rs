use std::env;

mod set1;
mod util;

fn main() {
    let chall_nr = env::args()
        .nth(1)
        .expect("Challenge number must be passed as argument")
        .parse()
        .expect("Challenge number must be an integer");

    match chall_nr {
        1 => set1::chall01::solve_chall01(),
        2 => set1::chall02::solve_chall02(),
        _ => unimplemented!("Unknown challenge number"),
    }
}
