mod termo;
use termo::*;

fn main() {
    let r = Solver::solve(&vec![
        (
            Word(['t', 'a', 'r', 's', 'o']),
            Pattern([
                Status::Wrong,
                Status::Place,
                Status::Place,
                Status::Wrong,
                Status::Wrong,
            ])
        ),
    ]);

    println!("{:?}", r);
}