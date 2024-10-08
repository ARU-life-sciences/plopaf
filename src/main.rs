mod paf;

use plopaf::{parse_args, run};

fn main() {
    let args = parse_args();
    let _ = run(args);
}
