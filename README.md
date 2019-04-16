# AOJ-Client

REST API Client for http://developers.u-aizu.ac.jp/index

## Example

```rust
use aoj_client::solution::FindAllRequest;
use aoj_client::Client;
use failure::Error;

fn main() -> Result<(), Error> {
    let client = Client::default();
    let solutions = client
        .solution_client()
        .find_all(FindAllRequest::default().set_page(0).set_size(10))?;
    for solution in solutions {
        println!("{} solved {}", solution.user_id, solution.problem_id);
    }
    Ok(())
}
```
