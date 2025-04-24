use std::vec;

use chrono::{NaiveDate, Weekday};
use clap::{
    Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};
use inquire::{
    CustomType, DateSelect, MultiSelect, Select, Text, min_length, required,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    validator::Validation,
};
use semver::Version;

use crate::generate::ProjectConfig;

const CLAP_STYLING: clap::builder::styling::Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default())
    .error(AnsiColor::Red.on_default());

const HELP_TEMPLATE: &str = r#"
welcome to use mm-cli {version}

{usage-heading} {usage}

{all-args}{after-help}
"#;

const AFTER_HELP: &str = r#"
help: https://www.workoss.com
"#;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "mm",
    version,
    author,
    about = "mm cli",
    help_template = HELP_TEMPLATE,
    after_help = AFTER_HELP,
    max_term_width = 120,
)]
#[command(next_line_help = false)]
#[command(propagate_version = true)]
#[command(styles = CLAP_STYLING)]
pub struct CliArgs {
    /// command to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// upgrade cli version (default: latest)
    Upgrade(UpgradeCommand),
    /// login to mm
    Login(LoginCommand),

    /// generate project
    Generate(GenerateCommand),
}

#[derive(Debug, Clone, Parser)]
pub struct UpgradeCommand {
    /// target version, if not set, will be latest
    pub version: Option<Version>,
    /// is force upgrade
    #[clap(long, default_value_t = false)]
    pub force: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct LoginCommand {
    /// username
    #[clap(long, short = 'u')]
    pub username: String,
    /// password
    #[clap(long, short = 'p')]
    pub password: String,
}

#[derive(Debug, Clone, Parser)]
pub struct GenerateCommand {
    /// template git url
    #[clap(long, short = 'g')]
    pub git: String,
}

impl GenerateCommand {
    pub fn prepare(&self) -> anyhow::Result<ProjectConfig> {
        // default project name exists +1
        let project_name = Text::new("project name: ")
            .with_validator(required!("项目名称不能为空"))
            // .with_initial_value("my_project")
            .with_placeholder("my_project")
            .with_help_message("输入或者修改，enter确认")
            .prompt()?;

        // let date = DateSelect::new("date: ")
        //     .with_starting_date(NaiveDate::from_ymd_opt(2021, 8, 1).unwrap())
        //     .with_min_date(NaiveDate::from_ymd_opt(2021, 8, 1).unwrap())
        //     .with_max_date(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap())
        //     .with_week_start(Weekday::Mon)
        //     .with_help_message("Possible flights will be displayed according to the selected date")
        //     .prompt()?;
        let date = CustomType::<String>::new("date: ")
            .with_validator(|input: &String| {
                if input.trim().is_empty() {
                    Ok(Validation::Invalid(
                        inquire::validator::ErrorMessage::Custom("日期不能为空".to_string()),
                    ))
                } else {
                    let msg = NaiveDate::parse_from_str(&input, "%Y-%m-%d")
                        .map(|_| Validation::Valid)
                        .unwrap_or(Validation::Invalid(
                            inquire::validator::ErrorMessage::Custom(
                                "日期格式错误 yyyy-MM-dd".to_string(),
                            ),
                        ));
                    Ok(msg)
                }
            })
            .with_help_message("输入或者修改，enter确认")
            .prompt()?;

        let language = Select::new("language: ", vec!["Java", "Rust", "React"])
            .with_page_size(5)
            .with_help_message("↑↓移动，enter确认")
            .prompt()?;

        let keywords = MultiSelect::new(
            "keywords: ",
            vec!["dev", "cli", "tool", "learn", "test", "command"],
        )
        .with_page_size(5)
        .with_help_message("↑↓移动，space 选中，→全选中，←全非选，enter确认")
        .with_default(&[0])
        .with_validator(min_length!(1, "keywords is required"))
        .prompt()?;

        Ok(ProjectConfig {
            name: project_name,
            language: language.to_string(),
            keywords: keywords.iter().map(|s| s.to_string()).collect(),
        })
    }
}

pub fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::LightRed);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
    //☑ ☐
    render_config.selected_checkbox = Styled::new("[x]").with_fg(Color::LightGreen);

    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("[ ]");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❗").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightGreen);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
