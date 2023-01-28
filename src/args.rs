use clap::Parser;

#[derive(Parser)]
// #[clap(author, version, about)]
#[clap(name = "leet_daily")]
#[clap(about = "Opens leetcode daily qn in browser, creates file, opens file in neovim unless arguments specified")]
pub struct LeetDailyArgs {
    /// do not open in browser
    #[clap(short, long, action)]
    pub browser: bool,

    /// do not create a file if not existing
    #[clap(short, long, action)]
    pub file: bool,

    /// do not open file in neovim
    #[clap(short, long, action)]
    pub vim: bool,
}
