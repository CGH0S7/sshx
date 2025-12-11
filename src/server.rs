use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: String,
}

impl Server {
    pub fn address(&self) -> String {
        format!("{}@{}", self.user, self.host)
    }

    pub fn to_ssh_args(&self) -> Vec<String> {
        let mut args = vec![self.address()];
        if !self.port.is_empty() {
            args.push("-p".to_string());
            args.push(self.port.clone());
        }
        args
    }

    pub fn to_mosh_args(&self) -> Vec<String> {
        let mut args = vec![];
        if !self.port.is_empty() {
            args.push("--ssh".to_string());
            args.push(format!("ssh -p {}", self.port));
        }
        args.push(self.address());
        args
    }

    pub fn to_copy_id_args(&self) -> Vec<String> {
        let mut args = vec![];
        if !self.port.is_empty() {
            args.push("-p".to_string());
            args.push(self.port.clone());
        }
        args.push(self.address());
        args
    }
}

