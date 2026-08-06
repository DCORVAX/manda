#![allow(clippy::cast_abs_to_unsigned)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_find)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::needless_return)]
#![allow(clippy::question_mark)]
#![allow(clippy::single_match)]
#![allow(clippy::single_range_in_vec_init)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::useless_conversion)]

use anyhow::{anyhow, Context};
use clap::builder::ValueParser;
use clap::{Parser, ValueEnum, ValueHint};
use clap_complete::{generate as generate_completion, shells, Generator as CompletionGenerator};
use config::{wezterm_version, ConfigHandle};
use mux::Mux;
use std::ffi::OsString;
use std::path::PathBuf;
use umask::UmaskSaver;
use wezterm_gui_subcommands::*;

mod ai_config;
mod assistant_config;
mod chat;
mod cli;
mod config_cmd;
mod config_tui;
mod doctor;
mod init;
mod manda_theme;
mod reset;
mod shell;
mod tui_core;
mod tui_splash;
mod update;
mod utils;

#[derive(Debug, Parser)]
#[command(
    about = "MANDA Terminal Emulator\nhttp://github.com/WILFREDY-X/manda",
    version = wezterm_version()
)]
pub struct Opt {
    /// Skip loading manda.lua
    #[arg(long, short = 'n')]
    skip_config: bool,

    /// Specify the configuration file to use, overrides the normal
    /// configuration file resolution
    #[arg(
        long,
        value_parser,
        conflicts_with = "skip_config",
        value_hint=ValueHint::FilePath
    )]
    config_file: Option<OsString>,

    /// Override specific configuration values
    #[arg(
        long = "config",
        name = "name=value",
        value_parser=ValueParser::new(name_equals_value),
        number_of_values = 1)]
    config_override: Vec<(String, String)>,

    #[command(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
    Fig,
}

impl CompletionGenerator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => shells::Bash.file_name(name),
            Shell::Elvish => shells::Elvish.file_name(name),
            Shell::Fish => shells::Fish.file_name(name),
            Shell::PowerShell => shells::PowerShell.file_name(name),
            Shell::Zsh => shells::Zsh.file_name(name),
            Shell::Fig => clap_complete_fig::Fig.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => shells::Bash.generate(cmd, buf),
            Shell::Elvish => shells::Elvish.generate(cmd, buf),
            Shell::Fish => shells::Fish.generate(cmd, buf),
            Shell::PowerShell => shells::PowerShell.generate(cmd, buf),
            Shell::Zsh => shells::Zsh.generate(cmd, buf),
            Shell::Fig => clap_complete_fig::Fig.generate(cmd, buf),
        }
    }
}

#[derive(Debug, Parser, Clone)]
enum SubCommand {
    #[command(
        name = "start",
        about = "Start the GUI, optionally running an alternative program [aliases: -e]",
        hide = true
    )]
    Start(StartCommand),

    /// Start the GUI in blocking mode. You shouldn't see this, but you
    /// may see it in shell completions because of this open clap issue:
    /// <https://github.com/clap-rs/clap/issues/1335>
    #[command(short_flag_alias = 'e', hide = true)]
    BlockingStart(StartCommand),

    #[command(name = "ai", about = "Manage AI settings")]
    Ai(ai_config::AiConfigCommand),

    #[command(
        name = "provider",
        about = "Select a MANDA AI provider preset or list the available ones"
    )]
    Provider(ai_config::provider::ProviderCommand),

    #[command(
        name = "chat",
        about = "Start the AI chat in this terminal (alias for `k`)"
    )]
    Chat(chat::ChatCommand),

    #[command(name = "config", about = "Configure MANDA settings")]
    Config(config_cmd::ConfigCommand),

    #[command(name = "init", about = "Initialize MANDA shell integration")]
    Init(init::InitCommand),

    #[command(
        name = "doctor",
        about = "Check MANDA shell integration, environment, and runtime health"
    )]
    Doctor(doctor::DoctorCommand),

    #[command(
        name = "update",
        about = "Download and install the latest MANDA release automatically"
    )]
    Update(update::UpdateCommand),

    #[command(
        name = "reset",
        about = "Reset MANDA shell integration and managed defaults"
    )]
    Reset(reset::ResetCommand),

    #[command(
        name = "cli",
        about = "Interact with experimental mux server",
        hide = true
    )]
    Cli(cli::CliCommand),

    #[command(
        name = "set-working-directory",
        about = "Advise the terminal of the current working directory by \
                 emitting an OSC 7 escape sequence",
        hide = true
    )]
    SetCwd(SetCwdCommand),

    #[cfg(feature = "remote")]
    #[command(name = "remote", about = "Show QR code to connect MANDA iOS app")]
    Remote,

    /// Generate shell completion information
    #[command(name = "shell-completion", hide = true)]
    ShellCompletion {
        /// Which shell to generate for
        #[arg(long, value_parser)]
        shell: Shell,
    },
}

use termwiz::escape::osc::OperatingSystemCommand;

#[derive(Debug, Parser, Clone)]
struct SetCwdCommand {
    /// The directory to specify.
    /// If omitted, will use the current directory of the process itself.
    #[arg(value_parser, value_hint=ValueHint::DirPath)]
    cwd: Option<OsString>,

    /// How to manage passing the escape through to tmux
    #[arg(long, value_parser)]
    tmux_passthru: Option<TmuxPassthru>,

    /// The hostname to use in the constructed file:// URL.
    /// If omitted, the system hostname will be used.
    #[arg(value_parser, value_hint=ValueHint::Hostname)]
    host: Option<OsString>,
}

impl SetCwdCommand {
    fn run(&self) -> anyhow::Result<()> {
        let mut cwd = std::env::current_dir()?;
        if let Some(dir) = &self.cwd {
            cwd.push(dir);
        }

        let mut url = url::Url::from_directory_path(&cwd)
            .map_err(|_| anyhow::anyhow!("cwd {} is not an absolute path", cwd.display()))?;
        let host = match self.host.as_ref() {
            Some(h) => h.clone(),
            None => hostname::get()?,
        };
        let host = host.to_str().unwrap_or("localhost");
        url.set_host(Some(host))?;

        let osc = OperatingSystemCommand::CurrentWorkingDirectory(url.into());
        let tmux = self.tmux_passthru.unwrap_or_default();
        let encoded = tmux.encode(osc.to_string());
        print!("{encoded}");
        if tmux.enabled() {
            // Tmux understands OSC 7 but won't automatically pass it through.
            // <https://github.com/tmux/tmux/issues/3127#issuecomment-1076300455>
            // Let's do it again explicitly now.
            print!("{osc}");
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, ValueEnum, Default)]
enum TmuxPassthru {
    Disable,
    Enable,
    #[default]
    Detect,
}

impl TmuxPassthru {
    fn is_tmux() -> bool {
        std::env::var_os("TMUX").is_some()
    }

    fn enabled(&self) -> bool {
        match self {
            Self::Enable => true,
            Self::Detect => Self::is_tmux(),
            Self::Disable => false,
        }
    }

    fn encode(&self, content: String) -> String {
        if self.enabled() {
            let mut result = "\u{1b}Ptmux;".to_string();
            for c in content.chars() {
                if c == '\u{1b}' {
                    // Quote the escape by doubling it up
                    result.push(c);
                }
                result.push(c);
            }
            result.push_str("\u{1b}\\");
            result
        } else {
            content
        }
    }
}

fn terminate_with_error_message(err: &str) -> ! {
    log::error!("{}; terminating", err);
    std::process::exit(1);
}

fn terminate_with_error(err: anyhow::Error) -> ! {
    terminate_with_error_message(&format!("{:#}", err));
}

fn main() {
    config::designate_this_as_the_main_thread();
    config::assign_error_callback(mux::connui::show_configuration_error_message);
    if let Err(e) = run() {
        terminate_with_error(e);
    }
    Mux::shutdown();
}

fn init_config(opts: &Opt) -> anyhow::Result<ConfigHandle> {
    config::common_init(
        opts.config_file.as_ref(),
        &opts.config_override,
        opts.skip_config,
    )
    .context("config::common_init")?;
    let config = config::configuration();
    config.update_ulimit()?;
    if let Some(value) = &config.default_ssh_auth_sock {
        std::env::set_var("SSH_AUTH_SOCK", value);
    }
    Ok(config)
}

fn run() -> anyhow::Result<()> {
    let saver = UmaskSaver::new();

    // Clap renders --help/--version during parse, so version info must be
    // assigned before Opt::parse() even when we skip full env bootstrap.
    config::assign_version_info(
        wezterm_version::wezterm_version(),
        wezterm_version::wezterm_target_triple(),
    );

    let opts = Opt::parse();

    let cmd = if let Some(cmd) = opts.cmd.as_ref().cloned() {
        Some(cmd)
    } else {
        Some(SubCommand::Start(StartCommand::default()))
    };

    let Some(cmd) = cmd else {
        return Ok(());
    };

    match cmd {
        SubCommand::Start(_) | SubCommand::BlockingStart(_) => {
            env_bootstrap::bootstrap();
            delegate_to_gui(saver)
        }
        SubCommand::Cli(cli) => {
            env_bootstrap::bootstrap();
            cli::run_cli(&opts, cli)
        }
        #[cfg(feature = "remote")]
        SubCommand::Remote => {
            let state = manda_remote::read_state()?;
            let output = if let Some(relay) = &state.tunnel_relay {
                manda_remote::render_relay_qr_terminal(relay, &state.token)
            } else {
                let host = lan_ip().unwrap_or_else(|| "127.0.0.1".to_string());
                manda_remote::render_qr_terminal(&host, state.port, &state.token)
            };
            println!("{output}");
            Ok(())
        }
        SubCommand::SetCwd(cmd) => cmd.run(),
        SubCommand::ShellCompletion { shell } => {
            use clap::CommandFactory;
            let mut cmd = Opt::command();
            let name = cmd.get_name().to_string();
            generate_completion(shell, &mut cmd, name, &mut std::io::stdout());
            Ok(())
        }
        SubCommand::Update(cmd) => cmd.run(),
        SubCommand::Config(cmd) => cmd.run(
            opts.config_file.as_ref().map(PathBuf::from),
            opts.config_file.clone(),
            opts.config_override.clone(),
            opts.skip_config,
        ),
        SubCommand::Init(cmd) => cmd.run(),
        SubCommand::Doctor(cmd) => cmd.run(),
        SubCommand::Reset(cmd) => cmd.run(),
        SubCommand::Ai(cmd) => cmd.run(
            opts.config_file.clone(),
            opts.config_override.clone(),
            opts.skip_config,
        ),
        SubCommand::Chat(cmd) => cmd.run(),
        SubCommand::Provider(cmd) => cmd.run(),
    }
}

fn delegate_to_gui(saver: UmaskSaver) -> anyhow::Result<()> {
    use std::process::Command;

    // Restore the original umask
    drop(saver);

    let exe_name = if cfg!(windows) {
        "manda-gui.exe"
    } else {
        "manda-gui"
    };

    let exe = resolve_gui_executable(exe_name)?;

    let mut cmd = Command::new(&exe);
    if cfg!(windows) {
        cmd.arg("--attach-parent-console");
    }

    cmd.args(std::env::args_os().skip(1));

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Clean up random fds, except when we're running in an AppImage.
        // AppImage relies on child processes keeping alive an fd that
        // references the mount point and if we close it as part of execing
        // the gui binary, the appimage gets unmounted before we can exec.
        if std::env::var_os("APPIMAGE").is_none() {
            portable_pty::unix::close_random_fds();
        }
        let res = cmd.exec();
        return Err(anyhow::anyhow!("failed to exec {cmd:?}: {res:?}"));
    }

    #[cfg(windows)]
    {
        let mut child = cmd.spawn()?;
        let status = child.wait()?;
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }
}

fn resolve_gui_executable(exe_name: &str) -> anyhow::Result<PathBuf> {
    let current_exe = std::env::current_exe()?;
    let mut candidates = Vec::new();

    if let Some(parent) = current_exe.parent() {
        candidates.push(parent.join(exe_name));
    }

    if let Ok(resolved_exe) = std::fs::canonicalize(&current_exe) {
        if let Some(parent) = resolved_exe.parent() {
            let resolved_candidate = parent.join(exe_name);
            if !candidates
                .iter()
                .any(|candidate| candidate == &resolved_candidate)
            {
                candidates.push(resolved_candidate);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        candidates.push(PathBuf::from("/Applications/Manda.app/Contents/MacOS").join(exe_name));
        candidates.push(
            config::HOME_DIR
                .join("Applications")
                .join("Manda.app")
                .join("Contents")
                .join("MacOS")
                .join(exe_name),
        );
    }

    if let Some(path) = candidates.iter().find(|path| path.exists()) {
        return Ok(path.clone());
    }

    candidates
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("unable to resolve GUI executable path"))
}

#[cfg(feature = "remote")]
fn lan_ip() -> Option<String> {
    use std::net::UdpSocket;
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    Some(sock.local_addr().ok()?.ip().to_string())
}
