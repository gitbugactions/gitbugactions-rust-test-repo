use gitbugactions_rust_test_repo::AppMath;

fn main() {
    let app_math = AppMath::new();
    println!("{}", app_math.sum(1, 2));
}
