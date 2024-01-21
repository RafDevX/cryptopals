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
        3 => set1::chall03::solve_chall03(),
        4 => set1::chall04::solve_chall04(),
        5 => set1::chall05::solve_chall05(),
        6 => set1::chall06::solve_chall06(),
        7 => set1::chall07::solve_chall07(),
        _ => unimplemented!("Unknown challenge number"),
    }
}
