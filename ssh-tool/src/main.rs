use clap::{Parser, Subcommand, ValueEnum};
use rpassword::prompt_password;
use std::process::Command;
use anyhow::{Result, anyhow};

#[derive(Parser)]
#[clap(name = "ssh-tool", about = "A CLI tool to SSH into different environments")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to an environment via SSH
    Connect {
        #[clap(value_enum)]
        env: Environment,
    },
}

#[derive(ValueEnum, Clone)]
enum Environment {
    QA,
    RFS,
    Prod,
}

impl Environment {
    fn get_ssh_details(&self) -> (&str, &str, &str) {
        match self {
            Environment::QA => ("yash.sakle@192.168.171.74", "7777", "QA"),
            Environment::RFS => ("yash.sakle@172.16.6.15", "7777", "RFS"),
            Environment::Prod => ("yash.sakle@172.16.3.21", "22", "Prod"),
        }
    }
}

fn execute_ssh(env: Environment) -> Result<()> {
    let (host, port, env_name) = env.get_ssh_details();
    let password = prompt_password(format!("Enter SSH password for {}: ", env_name))?;

    // Use sshpass to pass the password to the SSH command
    let status = Command::new("sshpass")
        .arg("-p")
        .arg(&password)
        .arg("ssh")
        .arg("-o")
        .arg("StrictHostKeyChecking=no") // Avoid host key verification prompt
        .arg("-p")
        .arg(port)
        .arg(host)
        .status()?;

    if !status.success() {
        return Err(anyhow!("SSH connection to {} failed", env_name));
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Connect { env } => {
            execute_ssh(env)?;
            println!("SSH session ended.");
        }
    }

    Ok(())
}