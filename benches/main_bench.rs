//! Main test benchmark, parameterized with environment variables and command line arguments.
//! Requires feature "_bench" to be enabled.

use bench_diff::bench_support::bench_with_claims::bench_with_claims_and_args;

fn main() {
    bench_with_claims_and_args();
}
