use std::fs::File;
use std::io::Read;
use std::str::FromStr;
mod error;
mod pwd;
pub mod singal;
mod utils;

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
pub struct SElinux {
    se_user: Option<String>,
    se_role: Option<String>,
    se_type: Option<String>,
    se_sensitivity_level: Option<String>,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Process {
    pub cmdline: Vec<String>,
    pub thread_num: u16,
    pub user: String,
    pub gid: Gid,
    pub selinux: SElinux,
    pub status: ProcessStat,
    pub sig_ign: Vec<singal::Signal>,
    pub sig_pnd: Vec<singal::Signal>,
    pub sig_blk: Vec<singal::Signal>,
    pub sig_cgt: Vec<singal::Signal>,
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

pub fn get_process_info(pid: u32) -> Result<Process, String> {
    let mut buf = String::new();
    let status_path = format!("/proc/{}/status", pid);
    File::open(&status_path)
        .and_then(|mut file| file.read_to_string(&mut buf))
        .unwrap();

    let uid = utils::get_status_value(&buf, "Uid")?
        .parse::<Uid>()
        .unwrap();
    let user = pwd::Passwd::getpwuid(uid).unwrap().username().to_owned();
    let gid = utils::get_status_value(&buf, "Gid")?
        .parse::<Gid>()
        .unwrap();
    let thread_num = utils::get_status_value(&buf, "Threads")?
        .parse::<u16>()
        .unwrap();
    let sig_ign = singal::get_ignoring_signals(&buf);
    let sig_blk = singal::get_block_signals(&buf);
    let sig_pnd = singal::get_pending_signals(&buf);
    let sig_cgt = singal::get_cgt_signals(&buf);
    let selinux = get_selinux_context(pid).unwrap();
    let cmdline = get_proccess_cmdline(pid);
    let status = utils::get_status_value(&buf, "State")?;
    let status = ProcessStat::from_str(&status).unwrap();
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
        ..Default::default()
    })
}

pub fn get_selinux_context(pid: u32) -> Result<SElinux, Box<dyn std::error::Error>> {
    let mut context = String::new();
    let attr_path = format!("/proc/{}/attr/current", pid);
    File::open(&attr_path)
        .and_then(|mut file| file.read_to_string(&mut context))
        .unwrap();
    let mut context = context.split(":");
    let se_user = context
        .next()
        .map(|s| s.trim().parse::<String>().ok())
        .flatten();
    let se_role = context
        .next()
        .map(|s| s.trim().parse::<String>().ok())
        .flatten();
    let se_type = context
        .next()
        .map(|s| s.trim().parse::<String>().ok())
        .flatten();
    let se_sensitivity_level = context.next().map(|s| s.parse::<String>().ok()).flatten();
    Ok(SElinux {
        se_user,
        se_role,
        se_type,
        se_sensitivity_level,
    })
}
pub fn get_proccess_cmdline(pid: u32) -> Vec<String> {
    let mut cmdline = String::new();
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    File::open(&cmdline_path)
        .and_then(|mut file| file.read_to_string(&mut cmdline))
        .unwrap();
    cmdline
        .split_whitespace()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>()
}
