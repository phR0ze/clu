//! `clu` provides automation for the Arch Linux ecosystem
mod logger;
use clap::{App, AppSettings, Arg, SubCommand};
use libclu::prelude::*;
use std::{env, ffi::OsString};
use witcher::prelude::*;

/// CLU providers a command line interface for libclu
#[derive(Debug, PartialEq)]
struct CLI;
impl CLI {
    /// Create a new instance of the cli with the given arguments
    pub fn new<T, U>(args: T) -> Result<Self>
    where
        T: Iterator<Item = U>,
        U: Into<OsString> + Clone,
    {
        // About strings
        let info_about = r"View package information

Examples:

  # View package info for the 'linux' package
  clu info linux
";

        let use_about = r"Persist configuration across runs

Examples:

  # Use and persist the 'base' profile
  clu use profile base

  # Use and persist a custom profile '~/foo.yaml'
  clu use profile ~/foo.yaml
";

        // Parse cli args
        let matches = App::new(APP_NAME)
            .version(&format!("v{}", APP_VERSION)[..])
            .about(APP_DESCRIPTION)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            // Global arguments
            // -----------------------------------------------------------------------------------------
            .arg(Arg::with_name("test").short("t").long("test").takes_value(false).help("Enable test mode"))
            .arg(Arg::with_name("debug").short("d").long("debug").takes_value(false).help("Enable debug logging"))
            .arg(Arg::with_name("quiet").short("q").long("quiet").takes_value(false).help("Disable all logging"))
            // log-level - configures the log level to use
            .arg(
                Arg::with_name("loglevel")
                    .long("log-level")
                    .value_name("NAME")
                    .takes_value(true)
                    .help("Sets the log level [error|warn|info|debug|trace] [default: info]"),
            )
            // config-dir - is where clu persists its configuration
            .arg(
                Arg::with_name("config_dir")
                    .long("config-dir")
                    .value_name("PATH")
                    .takes_value(true)
                    .help("Sets the config directory [default: $XDG_CONFIG_HOME/clu]"),
            )
            // data-dir - is where all repos are downloaded and all work is done
            .arg(
                Arg::with_name("data_dir")
                    .long("data-dir")
                    .value_name("PATH")
                    .takes_value(true)
                    .help("Sets the data directory [default: $XDG_DATA_HOME/clu]"),
            )
            // Version command
            // -----------------------------------------------------------------------------------------
            .subcommand(
                SubCommand::with_name("version")
                    .alias("v")
                    .alias("ver")
                    .about("Print version information"),
            )
            // Use command
            // -----------------------------------------------------------------------------------------
            .subcommand(
                SubCommand::with_name("use")
                    .about("Persist configuration across runs")
                    .long_about(use_about)
                    .subcommand(
                        SubCommand::with_name("profile")
                            .about("Use the given profile and persist the change")
                            .arg(
                                Arg::with_name("profile_arg")
                                    .index(1)
                                    .required(true)
                                    .value_names(&["NAME/PATH"])
                                    .help("Profile name or path to use and persist"),
                            ),
                    ),
            )
            // Info command
            // -----------------------------------------------------------------------------------------
            .subcommand(
                SubCommand::with_name("info")
                    .about("View package information")
                    .long_about(info_about)
                    .arg(
                        Arg::with_name("info_args")
                            .index(1)
                            .required(true)
                            .value_names(&["PACKAGE"])
                            .multiple(true)
                            .help("Package names to view information about"),
                    ),
            )
            // Remove command
            // -----------------------------------------------------------------------------------------
            .subcommand(
                SubCommand::with_name("remove")
                    .alias("rm")
                    .about("Remove various clu components")
                    .subcommand(SubCommand::with_name("config").about("Remove the persisted configuration"))
                    .subcommand(
                        SubCommand::with_name("repos")
                            .alias("repo")
                            .about("Remove indicated locally cloned repos")
                            .arg(
                                Arg::with_name("repos_arg")
                                    .index(1)
                                    .required(true)
                                    .value_names(&["all, aur, boot, config, profiles"])
                                    .multiple(true)
                                    .help("Repo name/s to remove"),
                            ),
                    ),
            )
            .get_matches_from_safe(args)
            .pass()?;

        // Initialize clu
        // ---------------------------------------------------------------------------------------------
        logger::init();

        // Configure clu
        let mut clu = Clu::new()
            .with_config_dir(matches.value_of("config_dir"))
            .wrap("failed to set clu's 'config_dir' option")?
            .with_data_dir(matches.value_of("data_dir"))
            .wrap("failed to set clu's 'data_dir' option")?
            .with_debug(matches.is_present("debug"))
            .with_quiet(matches.is_present("quiet"))
            .with_test(matches.is_present("test"));

        // Execute version
        // ---------------------------------------------------------------------------------------------
        if let Some(ref _matches) = matches.subcommand_matches("version") {
            println!("{}: {}", APP_NAME.cyan(), APP_DESCRIPTION.cyan());
            println!("{}", "--------------------------------------------------------".cyan());
            println!("{:<w$} {}", "Version:", APP_VERSION, w = 18);
            println!("{:<w$} {}", "Build Date:", APP_BUILD_DATE, w = 18);
            println!("{:<w$} {}", "Git Commit:", APP_GIT_COMMIT, w = 18);
        } else {
            clu.init().wrap("failed to initialize clu")?;
        }

        // // Execute use command before initializing to to update config first
        // // ---------------------------------------------------------------------------------------------
        // if let Some(ref _matches) = matches.subcommand_matches("use") {
        //     // Simply print out current persisted configuration
        //     return Err(PathError::parent_not_found("blah").into());
        // }

        // // Execute info
        // // ---------------------------------------------------------------------------------------------
        // if let Some(ref args) = matches.subcommand_matches("info") {
        //     clu.init()?;
        //     let pkgs = args.values_of_lossy("info_args").unwrap();
        //     clu.info(&pkgs)?;

        //     // match matches.subcommand() {
        //     //     ("info", Some(args)) => {
        //     //         let pkgs = args.values_of_lossy("info_args").unwrap();
        //     //         clu.info(&pkgs)?;
        //     //     }
        //     //     _ => fatal!("No sub-command specified\n{}", matches.usage()),
        //     // }
        // }

        // // Execute remove
        // // ---------------------------------------------------------------------------------------------
        // if let Some(ref matches) = matches.subcommand_matches("remove") {
        //     clu.init()?;
        //     let mut components = Vec::new();
        //     match matches.subcommand() {
        //         ("config", Some(_)) => {
        //             components.push(Component::Config);
        //         }
        //         _ => unreachable!(),
        //     }

        //     clu.remove(components)?;
        // }

        Ok(Self)
    }
}

fn main() {
    match CLI::new(env::args_os().into_iter()) {
        Ok(_) => 0,
        Err(err) => {
            match err.downcast_ref::<clap::Error>() {
                Some(clap) => println!("{}", clap),
                None => println!("{:?}", err),
            };
            1
        }
    };
}
