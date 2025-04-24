use std::process::exit;

use args::CliArgs;
use clap::Parser as _;
mod args;
mod generate;
fn main() -> anyhow::Result<()> {
    set_panic_handler();

    inquire::set_global_render_config(args::get_render_config());

    let args = CliArgs::parse();

    match args.command {
        args::Command::Upgrade(upgrade) => {
            println!("upgrade {:?}", upgrade);
            // let version = upgrade.version.unwrap_or_else(|| {
            //     let latest = get_latest_version().unwrap();
            //     println!("latest version: {}", latest);
            //     latest
            // });
            // upgrade(version, upgrade.force).unwrap();
        }
        args::Command::Login(login_command) => {
            println!("login {:?}", login_command);
            // login(login_command.username, login_command.password).unwrap();
        }
        args::Command::Generate(generate_command) => {
            println!("generate {:?}", generate_command);
            let project_config = generate_command.prepare()?;
            generate::generate(project_config)?;
        }
    }

    Ok(())
}

fn set_panic_handler() {
    std::panic::set_hook(Box::new(|pi| {
        log::error!("panic {}", pi.to_string());
        exit(-1);
    }));
}
