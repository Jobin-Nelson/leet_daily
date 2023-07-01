mod args;
mod leet_daily;
use args::LeetDailyArgs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = LeetDailyArgs::parse();

    let (current_date, daily_qn_link) = leet_daily::get_daily_qn_link()?;

    let file_path = leet_daily::create_file_path(&daily_qn_link);

    if !args.browser {
        leet_daily::open_link_in_browser(&daily_qn_link);
    }

    if !args.file {
        leet_daily::create_file(&file_path, &current_date, &daily_qn_link);
    }

    if !args.vim {
        leet_daily::open_file_in_vim(&file_path);
    }

    Ok(())
}
