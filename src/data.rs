use crate::{
    constants::{CLI_HELP_TEXT_WITHOUT_PROJECT_NOR_FLAG_OPTION_DESCRIPTIONS, VALID_FLAGS},
    utils::PEResult,
};
use colored::*;
use reqwest::Url;

#[derive(Debug)]
pub struct ProgramError {
    message: String,
}

// IMPORTANT! update enum values in tandem with constants::VALID_FLAGS
#[derive(Debug, PartialEq, Clone)]
pub enum Flag {
    Help,
}

pub struct Config {
    downloadables: Vec<Downloadable>,
    flags: Vec<Flag>,
}

struct Downloadable {
    url: Url,
}

impl Downloadable {
    fn build(url: String) -> PEResult<Self> {
        let url = Url::parse(&url);
        if url.is_ok() {
            Ok(Self { url: url.unwrap() })
        } else {
            Err(ProgramError::new(format!(
                "Failed to parse url: {}",
                url.unwrap_err().to_string()
            )))
        }
    }
}

impl Config {
    pub fn from<I: Iterator<Item = String>>(mut raw_args: I) -> PEResult<Self> {
        let mut downloadables = vec![];
        let mut flags: Vec<Flag> = vec![];

        while let Some(mut arg) = raw_args.next() {
            arg = arg.trim().to_lowercase();
            if arg.starts_with("-") {
                flags.push(Self::map_string_to_flag(arg)?);
            } else {
                downloadables.push(Downloadable::build(arg)?);
            }
        }

        Ok(Self {
            flags,
            downloadables,
        })
    }

    pub fn get_flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    fn map_string_to_flag(s: String) -> PEResult<Flag> {
        let flag = VALID_FLAGS.iter().find(|flag| flag.0 == s || flag.1 == s);

        if let Some(flag) = flag {
            Ok(flag.2.to_owned())
        } else {
            Err(ProgramError::new(format!("{s} is not a vlid flag")))
        }
    }
}

impl ProgramError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn msg(&self) -> &str {
        &self.message
    }
}

impl Flag {
    pub fn handle_help_flag(config: &Config) -> Result<(), ()> {
        if config.get_flags().contains(&Self::Help) {
            println!(
                "{CLI_HELP_TEXT_WITHOUT_PROJECT_NOR_FLAG_OPTION_DESCRIPTIONS}\n\n{}:\n{}\n\n",
                "Flags".blue(),
                VALID_FLAGS
                    .iter()
                    .enumerate()
                    .map(|(index, opt)| format!(
                        "{}. {} | {}: {}",
                        (index + 1).to_string().blue(),
                        opt.0.green(),
                        opt.1.green(),
                        opt.3
                    ))
                    .reduce(|acc_str, s| format!("{acc_str}\n{s}"))
                    .unwrap_or("".to_string())
            );
            Ok(())
        } else {
            Err(())
        }
    }
}
