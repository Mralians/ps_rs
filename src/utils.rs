#![allow(dead_code)]

use super::error::ProcessError;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
pub(super) fn get_proc_status_value(pid: u32, key: &str) -> Result<String, ProcessError> {
    let proc_status_path: PathBuf = format!("/proc/{pid}/status").into();
    let mut status = String::new();
    File::open(proc_status_path)
        .and_then(|mut f| f.read_to_string(&mut status))
        .map_err(ProcessError::Io)?;
    let value = status
        .lines()
        .find(|line| matches!(line.find(key), Some(0)))
        .ok_or(ProcessError::Match(format!(
            "{key} not found for PID {pid}"
        )))?
        .split_whitespace()
        .nth(1)
        .ok_or(ProcessError::Match(format!(
            "cannot get field 1 for key: {key}"
        )))?;
    Ok(value.to_owned())
}

pub(super) fn get_all_pids() -> Result<Vec<u32>, ProcessError> {
    let entries = fs::read_dir("/proc").map_err(ProcessError::Io)?;
    let all_pids = entries
        .filter_map(|dir| {
            let dir = dir.map_err(ProcessError::Io).ok()?;
            let filename = dir.file_name();
            let filename_str = filename.to_string_lossy();
            if filename_str.matches(char::is_numeric).count() == filename_str.len() {
                let pid = filename_str
                    .parse::<u32>()
                    .map_err(ProcessError::Parse)
                    .ok()?;
                Some(pid)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(all_pids)
}

pub(super) fn get_pids_by_name(s: &str) -> Result<Vec<u32>, ProcessError> {
    let all_pids = get_all_pids()?;
    let pids = all_pids
        .iter()
        .filter_map(|pid| {
            let pid_name = get_proc_status_value(*pid, "Name").ok()?;
            if pid_name.contains(s) {
                Some(*pid)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(pids)
}
