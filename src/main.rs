use std::env;

mod set1;
mod set2;
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
        8 => set1::chall08::solve_chall08(),

        9 => set2::chall09::solve_chall09(),
        10 => set2::chall10::solve_chall10(),
        11 => set2::chall11::solve_chall11(),
        12 => set2::chall12::solve_chall12(),
        13 => set2::chall13::solve_chall13(),
        14 => set2::chall14::solve_chall14(),
        15 => set2::chall15::solve_chall15(),

        _ => unimplemented!("Unknown challenge number"),
    }
}
