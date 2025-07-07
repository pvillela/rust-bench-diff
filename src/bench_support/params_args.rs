//! Module that supports test benchmarks, with the definition of target functions, test scenarios,
//! their parameterization, and passing of arguments from environment variables and the command line.

use crate::test_support::FnSpec;
use std::env;

pub struct BenchArgs {
    pub scale_name: String,
    pub fn_spec_pairs: Vec<(FnSpec, FnSpec)>,
    pub verbose: bool,
    pub nrepeats: usize,
    pub run_name: String,
}

fn cmd_line_args() -> Option<(usize, String)> {
    let mut args = std::env::args();

    let nrepeats = match args.nth(1) {
        Some(v) if v.ne("--bench") => v.parse::<usize>().unwrap_or_else(|_| {
            panic!("*** 1st argument, if provided, must be non-negative integer; was \"{v}\"")
        }),
        _ => return None,
    };

    let run_name = match args.next() {
        Some(v) if v.ne("--bench") => v,
        _ => String::new(),
    };

    Some((nrepeats, run_name))
}

pub fn get_args() -> BenchArgs {
    let (nrepeats, run_name) = cmd_line_args().unwrap_or((1, "".to_string()));

    let scale_name = env::var("SCALE_NAME").unwrap_or("micros_scale".into());

    let fn_spec_pairs: Vec<(FnSpec, FnSpec)> = {
        let fn_name_pairs_res = env::var("FN_NAME_PAIRS");
        match &fn_name_pairs_res {
            Ok(s) => s
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .split("|")
                .map(|x| {
                    let pair_v = x.split(",").collect::<Vec<_>>();
                    let err_msg = "*** properly formatted function name pair must contain one `,`: "
                        .to_string() + x;
                    assert!(pair_v.len() == 2, "{err_msg}");
                    (FnSpec::parse(pair_v[0]), FnSpec::parse(pair_v[1]))
                })
                .collect::<Vec<_>>(),
            Err(_) => panic!("FN_NAME_PAIRS environment variable is not defined"),
        }
    };

    let verbose: bool = {
        let verbose_str = env::var("VERBOSE").unwrap_or("false".into());
        verbose_str
            .parse()
            .expect("VERBOSE environment variable has invalid string representation of boolean")
    };

    BenchArgs {
        scale_name,
        fn_spec_pairs,
        verbose,
        nrepeats,
        run_name,
    }
}
