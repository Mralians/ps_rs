#![allow(dead_code)]

use std::fs::File;
use std::io::{Read, Result as ioResult};
use std::path::PathBuf;
type Uid = u32;
type Gid = u32;

/// A struct representing an entry in the `/etc/passwd` file.
#[derive(Debug, Default)]
pub struct Passwd {
    pw_name: String,
    pw_passwd: String,
    pw_uid: Uid,
    pw_gid: Gid,
    pw_gecos: String,
    pw_dir: PathBuf,
    pw_shell: PathBuf,
}
/// Parses the `/etc/passwd` file and returns a vector of `Passwd` structs.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
fn parse_passwd() -> ioResult<Vec<Passwd>> {
    let mut passwd = String::new();
    File::open("/etc/passwd").and_then(|mut f| f.read_to_string(&mut passwd))?;
    Ok(passwd
        .lines()
        .filter_map(|f| {
            let mut records = f.split(':');
            let pw_name = records.next()?.parse().ok()?;
            let pw_passwd = records.next()?.parse().ok()?;
            let pw_uid = records.next()?.parse().ok()?;
            let pw_gid = records.next()?.parse().ok()?;
            let pw_gecos = records.next()?.parse().ok()?;
            let pw_dir = records.next()?.parse().ok()?;
            let pw_shell = records.next()?.parse().ok()?;
            Some(Passwd {
                pw_name,
                pw_passwd,
                pw_uid,
                pw_gid,
                pw_gecos,
                pw_dir,
                pw_shell,
            })
        })
        .collect())
}

impl Passwd {
    /// Retrieve the user database entry for the given username.
    ///
    /// # Examples
    ///
    /// ```
    /// use linux::sysinfo::pwd::Passwd;
    /// let passwd = match Passwd::getpwnam("root") {
    ///        Some(user) => user,
    ///        None => panic!("user not found!"),
    /// };
    /// ```
    #[inline]
    pub fn getpwnam(username: &str) -> Option<Passwd> {
        let passwd = parse_passwd().unwrap();
        passwd.into_iter().find(|passwd| passwd.pw_name == username)
    }
    /// Retrieve the user database entry for the given user id.
    /// # Examples
    /// ```
    /// use linux::sysinfo::pwd::Passwd;
    /// let passwd = match Passwd::getpwuid(0) {
    ///        Some(user) => user,
    ///        None => panic!("user not found!"),
    /// };
    /// ```
    #[inline]
    pub fn getpwuid(uid: Uid) -> Option<Passwd> {
        let passwd = parse_passwd().unwrap();
        passwd.into_iter().find(|passwd| passwd.pw_uid == uid)
    }

    #[inline]
    pub fn username(&self) -> &str {
        self.pw_name.as_ref()
    }

    #[inline]
    pub fn password(&self) -> &str {
        self.pw_passwd.as_ref()
    }

    #[inline]
    pub fn uid(&self) -> u32 {
        self.pw_uid
    }

    #[inline]
    pub fn gid(&self) -> u32 {
        self.pw_gid
    }

    #[inline]
    pub fn description(&self) -> &str {
        self.pw_gecos.as_ref()
    }

    #[inline]
    pub fn home(&self) -> &PathBuf {
        &self.pw_dir
    }
    #[inline]
    pub fn shell(&self) -> &PathBuf {
        &self.pw_shell
    }
}
