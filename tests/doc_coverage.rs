use cargo_tarpaulin::config::{Config, RunType};
use cargo_tarpaulin::launch_tarpaulin;
use std::env;
use std::time::Duration;

#[test]
fn doc_test_coverage() {
    let mut config = Config::default();
    config.verbose = true;
    config.test_timeout = Duration::from_secs(60);
    let mut test_dir = env::current_dir().unwrap();
    test_dir.push("tests");
    test_dir.push("data");
    test_dir.push("doc_coverage");
    env::set_current_dir(test_dir.clone()).unwrap();
    config.manifest = test_dir.clone();
    config.manifest.push("Cargo.toml");

    config.run_types = vec![RunType::Doctests];

    let (res, ret) = launch_tarpaulin(&config).unwrap();

    assert_eq!(ret, 0);
    assert!(res.total_covered() > 0);
    assert_eq!(res.total_covered(), res.total_coverable());

    config.run_types = vec![RunType::Tests];

    let (res, ret) = launch_tarpaulin(&config).unwrap();

    assert_eq!(ret, 0);
    assert_eq!(res.total_covered(), 0);
}
