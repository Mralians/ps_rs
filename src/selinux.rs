use std::fs::File;
use std::io::Read;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SElinux {
    se_user: Option<String>,
    se_role: Option<String>,
    se_type: Option<String>,
    se_sensitivity_level: Option<String>,
}

pub fn get_selinux_context(pid: u32) -> Result<SElinux, Box<dyn std::error::Error>> {
    let mut context = String::new();
    let attr_path = format!("/proc/{pid}/attr/current");
    File::open(attr_path)
        .and_then(|mut file| file.read_to_string(&mut context))
        .unwrap();
    let mut context = context.split(':');
    let se_user = context.next().and_then(|s| s.trim().parse::<String>().ok());
    let se_role = context.next().and_then(|s| s.trim().parse::<String>().ok());
    let se_type = context.next().and_then(|s| s.trim().parse::<String>().ok());
    let se_sensitivity_level = context.next().and_then(|s| s.trim().parse::<String>().ok());
    Ok(SElinux {
        se_user,
        se_role,
        se_type,
        se_sensitivity_level,
    })
}
