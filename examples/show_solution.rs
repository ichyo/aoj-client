use aoj_client::client;
use aoj_client::solution::findallrequest;
use failure::error;

fn main() -> result<(), error> {
    let client = client::default();
    let solutions = client
        .solution_client()
        .find_all(findallrequest::default().set_page(0).set_size(10))?;
    for solution in solutions {
        println!("{} solved {}", solution.user_id, solution.problem_id);
    }
    ok(())
}
