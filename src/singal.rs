use std::fs::File;

use super::utils::get_status_value;
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Signal {
    SIGHUP,
    SIGINT,
    SIGQUIT,
    SIGILL,
    SIGTRAP,
    SIGABRT,
    SIGBUS,
    SIGFPE,
    SIGKILL,
    SIGUSR1,
    SIGSEGV,
    SIGUSR2,
    SIGPIPE,
    SIGALRM,
    SIGTERM,
    SIGSTKFLT,
    SIGCHLD,
    SIGCONT,
    SIGSTOP,
    SIGTSTP,
    SIGTTIN,
    SIGTTOU,
    SIGURG,
    SIGXCPU,
    SIGXFSZ,
    SIGVTALRM,
    SIGPROF,
    SIGWINCH,
    SIGIO,
    SIGPWR,
    SIGSYS,
    SIGRTMIN,
    UNUSED,
    SIGRTMAX,
    #[default]
    UNKNOWN,
}
impl TryFrom<u32> for Signal {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SIGHUP),
            2 => Ok(Self::SIGINT),
            3 => Ok(Self::SIGQUIT),
            4 => Ok(Self::SIGILL),
            5 => Ok(Self::SIGTRAP),
            6 => Ok(Self::SIGABRT),
            7 => Ok(Self::SIGBUS),
            8 => Ok(Self::SIGFPE),
            9 => Ok(Self::SIGKILL),
            10 => Ok(Self::SIGUSR1),
            11 => Ok(Self::SIGSEGV),
            12 => Ok(Self::SIGUSR2),
            13 => Ok(Self::SIGPIPE),
            14 => Ok(Self::SIGALRM),
            15 => Ok(Self::SIGTERM),
            16 => Ok(Self::SIGSTKFLT),
            17 => Ok(Self::SIGCHLD),
            18 => Ok(Self::SIGCONT),
            19 => Ok(Self::SIGSTOP),
            20 => Ok(Self::SIGTSTP),
            21 => Ok(Self::SIGTTIN),
            22 => Ok(Self::SIGTTOU),
            23 => Ok(Self::SIGURG),
            24 => Ok(Self::SIGXCPU),
            25 => Ok(Self::SIGXFSZ),
            26 => Ok(Self::SIGVTALRM),
            27 => Ok(Self::SIGPROF),
            28 => Ok(Self::SIGWINCH),
            29 => Ok(Self::SIGIO),
            30 => Ok(Self::SIGPWR),
            31 => Ok(Self::SIGSYS),
            32 => Ok(Self::SIGRTMIN),
            33..=63 => Ok(Self::UNUSED),
            64 => Ok(Self::SIGRTMAX),
            _ => Err("UNKNOWN".to_owned()),
        }
    }
}
pub(super) fn get_ignoring_signals(status: &str) -> Vec<Signal> {
    let mut signals: Vec<Signal> = vec![];
    let sig_ign = get_status_value(status, "SigIgn").unwrap();
    let value = u64::from_str_radix(&sig_ign, 16).unwrap();
    let mut mask = 1_u64;
    for i in 0..64 {
        if (value & mask) != 0 {
            signals.push(i.try_into().unwrap())
        }
        mask <<= 1;
    }
    signals
}
pub(super) fn get_pending_signals(status: &str) -> Vec<Signal> {
    let mut signals: Vec<Signal> = vec![];
    let sig_pnd = get_status_value(status, "SigPnd").unwrap();
    let value = u64::from_str_radix(&sig_pnd, 16).unwrap();
    let mut mask = 1_u64;
    for i in 1..=64 {
        if (value & mask) != 0 {
            signals.push(i.try_into().unwrap())
        }
        mask <<= 1;
    }
    signals
}
pub(super) fn get_block_signals(status: &str) -> Vec<Signal> {
    let mut signals: Vec<Signal> = vec![];
    let sig_blk = get_status_value(status, "SigBlk").unwrap();
    let value = u64::from_str_radix(&sig_blk, 16).unwrap();
    let mut mask = 1_u64;
    for i in 1..=64 {
        if (value & mask) != 0 {
            signals.push(i.try_into().unwrap())
        }
        mask <<= 1;
    }
    signals
}
pub(super) fn get_cgt_signals(status: &str) -> Vec<Signal> {
    let mut signals: Vec<Signal> = vec![];
    let sig_cgt = get_status_value(status, "SigCgt").unwrap();
    println!("{sig_cgt}");
    let value = u64::from_str_radix(&sig_cgt, 16).unwrap();
    let mut mask = 1_64;
    for i in 1..=64 {
        if (value & mask) != 0 {
            signals.push(i.try_into().unwrap())
        }
        mask <<= 1;
    }
    signals
}
