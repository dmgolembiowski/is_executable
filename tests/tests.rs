extern crate diff;
extern crate is_executable;

use is_executable::{ is_executable, is_permitted };

#[cfg(unix)]
mod unix {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::process::Command;

    use super::*;

    #[test]
    fn cargo_readme_up_to_date() {
        if env::var("CI").is_ok() {
            return;
        }

        println!("Checking that `cargo readme > README.md` is up to date...");

        let expected = Command::new("cargo")
            .arg("readme")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("should run `cargo readme` OK")
            .stdout;
        let expected = String::from_utf8_lossy(&expected);

        let actual = {
            let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
                .expect("should open README.md file");
            let mut s = String::new();
            file.read_to_string(&mut s)
                .expect("should read contents of file to string");
            s
        };

        if actual != expected {
            println!();
            println!("+++ expected README.md");
            println!("--- actual README.md");
            for d in diff::lines(&expected, &actual) {
                match d {
                    diff::Result::Left(l) => println!("+{}", l),
                    diff::Result::Right(r) => println!("-{}", r),
                    diff::Result::Both(b, _) => println!(" {}", b),
                }
            }
            panic!("Run `cargo readme > README.md` to update README.md")
        }
    }

    #[test]
    fn executable() {
        assert!(is_executable("./tests/i_am_executable"));
    }

    #[test]
    fn executable_symlink() {
        assert!(is_executable("./tests/i_am_executable_and_symlink"));
    }

    #[test]
    fn not_executable_symlink() {
        assert!(!is_executable("./tests/i_am_not_executable_and_symlink"));
    }

    #[test]
    fn not_executable_directory() {
        assert!(!is_executable("."));
    }

    #[test]
    fn permitted_by_membership() {
        // `chmod 670 ./tests/i_am_permitted_by_group_membership`
        let path: &str = "./tests/i_am_permitted_by_group_membership";
        {
            let check = is_permitted(path).ok().unwrap();
            assert_eq!(
                check,
                ::std::path::PathBuf::from(path),
                "Testing whether the file's GID is in the set of the user's groups" 
            );
        }
    }

    #[test]
    fn permitted_by_ownership() {
        // `chmod 700 ./tests/i_am_permitted_by_file_ownership`
        let path: &str = "./tests/i_am_permitted_by_file_ownership";
        {
            let check = is_permitted(path).ok().unwrap();
            assert_eq!(
                check,
                ::std::path::PathBuf::from(path),
                "Testing whether the file's UID bits exclusively allow owners to execute
                this file, and that the UID of this user is happens to be the file's owner." 
            );
        }
    }

}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    #[test]
    fn executable() {
        assert!(is_executable("C:\\Windows\\explorer.exe"));
    }

    #[test]
    fn by_extension() {
        assert!(is_executable("./tests/i_am_executable_on_windows.bat"));
    }

}

#[test]
fn not_executable() {
    assert!(!is_executable("./tests/i_am_not_executable"));
}

#[test]
fn non_existant() {
    assert!(!is_executable("./tests/this-file-does-not-exist"));
}
