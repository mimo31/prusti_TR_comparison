use prusti_contracts::*;

#[requires(time_credits(1))]
fn main() {}

#[requires(time_credits(n + 1))]
#[ensures(time_receipts(n + 1))]
fn do_work(n: usize) {
    let mut i = 0;
    while i < n {
        body_invariant!(time_receipts(i) & time_credits(n - i));
        i += 1;
    }
}
