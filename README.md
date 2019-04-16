# AOJ-Client

REST API Client for http://developers.u-aizu.ac.jp/index

## Example

```rust
use aoj_client::solution::findallrequest;
use aoj_client::client;
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
```
