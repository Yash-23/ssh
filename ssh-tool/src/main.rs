use clap::{Parser, Subcommand, ValueEnum};
use rpassword::prompt_password;
use std::process::Command;
use anyhow::{Result, anyhow};

#[derive(Parser)]
#[clap(name = "ssh-tool", about = "A CLI tool to SSH into Kubernetes services")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a specific service pod via SSH (QA/RFS) or custom method (Prod)
    Connect {
        #[clap(value_enum)]
        env: Environment,
        service_type: String,
        version: String,
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

fn execute_ssh(env: Environment, service_type: String, version: String) -> Result<()> {
    let service_name = format!("{}-{}", service_type, version);
    let (jump_host, jump_port, env_name) = env.get_ssh_details();
    let password = prompt_password(format!("Enter SSH password for {}: ", env_name))?;

    let ssh_command = match env {
        Environment::QA | Environment::RFS => {
            format!("ssh-pods {}", service_name)
        }
        Environment::Prod => {
            // Placeholder for Prod-specific logic
            return Err(anyhow!(
                "Prod environment is not yet implemented. Please add custom connection logic here."
            ));
        }
    };

    let status = Command::new("sshpass")
        .arg("-p")
        .arg(&password)
        .arg("ssh")
        .arg("-t") // Force TTY allocation for interactive sessions
        .arg("-o")
        .arg("StrictHostKeyChecking=no")
        .arg("-p")
        .arg(jump_port)
        .arg(jump_host)
        .arg(&ssh_command)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Connection to {} or service {} failed", env_name, service_name));
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Connect { env, service_type, version } => {
            execute_ssh(env, service_type, version)?;
            println!("Session ended.");
        }
    }

    Ok(())
}