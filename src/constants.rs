use crate::data::Flag;

type Description = &'static str;
type ShortForm = &'static str;
type LongForm = &'static str;

/// The only flags you can pass to the CLI, along with their short forms, corresponding Flag enums, and descriptions.
/// Some flags only have an effect when passed with certain options. In these cases other non compatible flags will be completely egnored.
/// `flag = (long_form, short_form, Flag, description)`.
pub const VALID_FLAGS: [(LongForm, ShortForm, Flag, Description); 1] = [
    ("--help", "-h", Flag::Help, "Show CLI help. If passed with an option, shows option description and optional flags with their descriptions."),
];

pub const CLI_HELP_TEXT_WITHOUT_PROJECT_NOR_FLAG_OPTION_DESCRIPTIONS: &str = "Blaze-Downloader CLI HELP:\nThis CLI program is used to download files from the internet, just pass it a url.\n\nUSAGE:\nrun with: <url> <flags>";
