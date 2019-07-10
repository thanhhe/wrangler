use assert_cmd::prelude::*;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::process::{Child, Command, Stdio};

// Does not work on Windows because of the home_dir resolution
// (https://stebalien.github.io/doc/term/dirs/fn.home_dir.html) which can't be
// overridden like we do for Linux or MacOS.
#[cfg(not(target_os = "windows"))]
#[test]
fn it_generates_the_config() {
    let fake_home_dir = env::current_dir()
        .expect("could not retrieve cwd")
        .join("it_generates_the_config");
    let cmd = config_with_home(fake_home_dir.to_str().unwrap());
    let mut stdin = cmd.stdin.unwrap();

    write!(stdin, "a\n").unwrap(); // email
    write!(stdin, "b\n").unwrap(); // api_key

    let mut buffer = "".to_string();
    let mut stdout = cmd.stdout.unwrap();
    stdout
        .read_to_string(&mut buffer)
        .expect("could not read output");
    assert!(buffer.contains("Enter email: \nEnter api key: \n Successfully configured."));

    let config_file = fake_home_dir
        .join(".wrangler")
        .join("config")
        .join("default.toml");

    let config = fs::read_to_string(&config_file)
        .expect(&format!("could not read config at {:?}", &config_file));
    assert_eq!(
        config,
        r#"email = "a"
api_key = "b"
"#
    );

    fs::remove_dir_all(&fake_home_dir).expect("could not delete dir");
}

fn config_with_home(home_dir: &str) -> Child {
    let mut wrangler = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    wrangler
        .arg("config")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .env("HOME", home_dir)
        .spawn()
        .unwrap()
}
