use std::{env, io::Write, path::Path};

use crate::{
    constants::{CLI_HELP_TEXT_WITHOUT_PROJECT_NOR_FLAG_OPTION_DESCRIPTIONS, VALID_FLAGS},
    utils::{green_log, PEResult},
    yellow_log,
};
use colored::*;
use reqwest::Url;
use tokio::fs;

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

pub struct Downloadable {
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

    pub async fn download(self) -> PEResult {
        match reqwest::get(self.url.as_str()).await {
            Ok(res) => {
                green_log("\ndownloading..\n");
                Self::save_to_file(res).await
            }
            Err(e) => Err(ProgramError::new(format!(
                "Failed to download resource at \"{}\". {e}",
                self.url.as_str()
            ))),
        }
    }

    async fn save_to_file(mut res: reqwest::Response) -> PEResult<()> {
        let dir_path = &env::current_dir()
            .expect("should be able to get cwd")
            .join("downloads");

        match fs::create_dir(dir_path).await {
            Ok(_) => {}
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => {
                    return Err(ProgramError::new(format!(
                        "failed to create downloads dir: {e}"
                    )))
                }
            },
        };

        yellow_log(format!("{:#?}", res).as_str());

        let file_name = &mut format!("file-[{}]", res.url());
        *file_name = res
            .url()
            .to_string()
            .split("/")
            .last()
            .unwrap_or(file_name)
            .to_string();
        let f_path = &dir_path.join(Path::new(&file_name));

        match fs::File::create(f_path).await {
            Ok(f) => {
                green_log("\nwriting to file..\n");
                let mut f = f.into_std().await;
                while let Ok(Some(chunk)) = res.chunk().await {
                    f.write(&chunk).expect(
                        format!(
                            "should have been able to write to file: {}",
                            f_path
                                .as_path()
                                .to_str()
                                .expect("should be able to parse file path")
                        )
                        .as_str(),
                    );
                }

                Ok(())
            }
            Err(e) => Err(ProgramError::new(format!(
                "failed to create file for downloaded content: {e}"
            ))),
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

    pub fn get_downloadables(self) -> Vec<Downloadable> {
        self.downloadables
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
