use std::fs::read_to_string;
use std::fs::canonicalize;
use std::io::Result;
use std::path::PathBuf;
use std::cmp::max;

// Recurrence relation
// max of remove from one or the other or both if the last
// character doesn't match
//
// Otherwise, remove the last character from both, add
// one to the result, and keep going
fn lcs(str1: &str, str2: &str) -> i32 {
    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();

    if str1.len() == 0 || str2.len() == 0 {
        return 0;
    }

    let rows = str1.len();
    let cols = str2.len();

    const UNINITIALIZED: i32 = -1;

    let mut dp = vec![vec![UNINITIALIZED; cols + 1]; rows + 1];

    for r in 0..rows+1 {
        dp[r][0] = 0;
    }

    for c in 0..cols+1 {
        dp[0][c] = 0;
    }

    for r in 1..rows+1 {
        for c in 1..cols+1 {
            if str1[r-1] as char == str2[c-1] as char {
                dp[r][c] = dp[r-1][c-1] + 1;

            } else {
                dp[r][c] = max(dp[r-1][c], dp[r][c-1]);
            }
        }
    }

    for r in 0..rows+1 {
        for c in 0..cols+1 {
            print!("{} ", dp[r][c]);
        }
        println!();
    }

    return dp[rows][cols];
}

fn main() -> Result<()> {
    let f1 = PathBuf::from("./diffs/file1.txt");
    let f2 = PathBuf::from("./diffs/file2.txt");

    println!("{:?}", canonicalize(&f1));
    println!("{:?}", canonicalize(&f2));

    let f1_contents = read_to_string(f1)?;
    let f2_contents = read_to_string(f2)?;

    println!("f1_contents: {f1_contents}");
    println!("f2_contents: {f2_contents}");

    println!("{}", lcs(&f1_contents, &f2_contents));

    Ok(())
}