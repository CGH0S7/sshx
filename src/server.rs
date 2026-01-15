use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: String,
    #[serde(default)]
    pub jump_host: String, // 跳板机，格式: user@host:port 或 user@host
}

impl Server {
    pub fn address(&self) -> String {
        format!("{}@{}", self.user, self.host)
    }

    pub fn to_ssh_args(&self) -> Vec<String> {
        let mut args = vec![];
        // 添加跳板机参数
        if !self.jump_host.is_empty() {
            args.push("-J".to_string());
            args.push(self.jump_host.clone());
        }
        args.push(self.address());
        if !self.port.is_empty() {
            args.push("-p".to_string());
            args.push(self.port.clone());
        }
        args
    }

    pub fn to_mosh_args(&self) -> Vec<String> {
        let mut args = vec![];
        let mut ssh_opts = String::new();
        if !self.port.is_empty() {
            ssh_opts.push_str(&format!("-p {}", self.port));
        }
        if !self.jump_host.is_empty() {
            if !ssh_opts.is_empty() {
                ssh_opts.push(' ');
            }
            ssh_opts.push_str(&format!("-J {}", self.jump_host));
        }
        if !ssh_opts.is_empty() {
            args.push("--ssh".to_string());
            args.push(format!("ssh {}", ssh_opts));
        }
        args.push(self.address());
        args
    }

    pub fn to_copy_id_args(&self) -> Vec<String> {
        let mut args = vec![];
        // 添加跳板机参数
        if !self.jump_host.is_empty() {
            args.push("-o".to_string());
            args.push(format!("ProxyJump={}", self.jump_host));
        }
        if !self.port.is_empty() {
            args.push("-p".to_string());
            args.push(self.port.clone());
        }
        args.push(self.address());
        args
    }

    pub fn to_sftp_args(&self) -> Vec<String> {
        let mut args = vec![];
        if !self.jump_host.is_empty() {
            args.push("-J".to_string());
            args.push(self.jump_host.clone());
        }
        if !self.port.is_empty() {
            args.push("-P".to_string());
            args.push(self.port.clone());
        }
        args.push(self.address());
        args
    }
}

