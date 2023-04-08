#![allow(clippy::redundant_closure)]
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use super::error::ProcessError;
use super::pwd;
use super::selinux::{self, SElinux};
use super::singal::{self, Signal};
use super::utils;

type Uid = u32;
type Gid = u32;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ProcessStat {
    Running,
    Sleeping,
    Zombie,
    #[default]
    UnkownStat,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Process {
    pub cmdline: Vec<String>,
    pub thread_num: u16,
    pub user: String,
    pub gid: Gid,
    pub selinux: SElinux,
    pub status: ProcessStat,
    pub sig_ign: Vec<Signal>,
    pub sig_pnd: Vec<Signal>,
    pub sig_blk: Vec<Signal>,
    pub sig_cgt: Vec<Signal>,
}
impl FromStr for ProcessStat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" | "sleeping" => Ok(Self::Sleeping),
            "R" | "running" => Ok(Self::Running),
            "Z" | "zombie" => Ok(Self::Zombie),
            _ => Err("Unkown state".to_owned()),
        }
    }
}

pub fn get_process_info(pid: u32) -> Result<Process, ProcessError> {
    let uid = utils::get_proc_status_value(pid, "Uid")?
        .parse::<Uid>()
        .unwrap();
    let user = pwd::Passwd::getpwuid(uid).unwrap().username().to_owned();
    let gid = utils::get_proc_status_value(pid, "Gid")?
        .parse::<Gid>()
        .unwrap();
    let thread_num = utils::get_proc_status_value(pid, "Threads")?
        .parse::<u16>()
        .unwrap();
    let sig_ign = singal::get_ignoring_signals(pid)?;
    let sig_blk = singal::get_block_signals(pid)?;
    let sig_pnd = singal::get_pending_signals(pid)?;
    let sig_cgt = singal::get_cgt_signals(pid)?;
    let selinux = selinux::get_selinux_context(pid).unwrap(); // FIXME: fix error handling
    let cmdline = get_proccess_cmdline(pid)?;
    let status = utils::get_proc_status_value(pid, "State")?;
    let status = ProcessStat::from_str(&status).unwrap(); //FIXME: fix error handling
    Ok(Process {
        user,
        gid,
        status,
        thread_num,
        selinux,
        cmdline,
        sig_ign,
        sig_blk,
        sig_pnd,
        sig_cgt,
    })
}
pub fn get_proccess_cmdline(pid: u32) -> Result<Vec<String>, ProcessError> {
    let mut cmdline = String::new();
    let cmdline_path = format!("/proc/{pid}/cmdline");
    File::open(cmdline_path)
        .and_then(|mut file| file.read_to_string(&mut cmdline))
        .map_err(|error| ProcessError::Io(error))?;
    let cmd = cmdline
        .split_whitespace()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    Ok(cmd)
}
