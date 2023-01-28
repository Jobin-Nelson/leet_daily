use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod args;
use args::LeetDailyArgs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = LeetDailyArgs::parse();

    let (current_date, daily_qn_link) = get_daily_qn_link().await?;

    let file_path = create_file_path(&daily_qn_link);

    if !args.browser {
        open_link_in_browser(&daily_qn_link);
    }

    if !args.file {
        create_file(&file_path, &current_date, &daily_qn_link);
    }

    if !args.vim {
        open_file_in_vim(&file_path);
    }

    Ok(())
}

async fn get_daily_qn_link() -> Result<(String, String), reqwest::Error> {
    let query_url = "https://leetcode.com/graphql";

    let query = HashMap::from([
        ("query",  "query questionOfToday {\n\tactiveDailyCodingChallengeQuestion {\n\t\tdate\n\t\tlink\n\t}\n}\n"),
        ("operationName", "questionOfToday")
    ]);

    let client = reqwest::Client::new();
    let data: serde_json::Value = client
        .post(query_url)
        .json(&query)
        .send()
        .await?
        .json()
        .await?;

    let base_url = String::from("https://leetcode.com");

    let current_date = data["data"]["activeDailyCodingChallengeQuestion"]["date"]
        .as_str()
        .unwrap()
        .to_owned();
    let relative_link = data["data"]["activeDailyCodingChallengeQuestion"]["link"]
        .as_str()
        .unwrap();
    let daily_qn_link = base_url + relative_link;

    Ok((current_date, daily_qn_link))
}

fn open_link_in_browser(daily_qn_link: &String) -> () {
    Command::new("brave")
        .arg(daily_qn_link)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Could not open link in brave browser");
}

fn create_file_path(daily_qn_link: &String) -> PathBuf {
    let now = chrono::Utc::now();

    let year = now.format("%Y").to_string();

    let month = now
        .format("%B")
        .to_string()
        .to_ascii_lowercase();

    let mut file_path: PathBuf = [
        "/home/jobin/playground/learn/competitive_programming",
        &year,
        &month,
    ]
    .iter()
    .collect();
    fs::create_dir_all(file_path.as_path()).expect("Could not create directory");
    let file_name = Path::new(daily_qn_link).file_name().unwrap();
    file_path.push(file_name);
    file_path.set_extension("py");

    file_path
}

fn create_file(file_path: &PathBuf, current_date: &String, daily_qn_link: &String) -> () {
    if file_path.exists() {
        println!("File already exist at {}", file_path.display());
        return ();
    }

    let content = format!(
        "\
'''
Created Date: {}
Qn:
Link: {}
Notes:
'''
def main():
    pass

if __name__ == '__main__':
",
        current_date, daily_qn_link
    );

    let mut f = fs::File::create(file_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    println!("File created at {}", file_path.display());
}

fn open_file_in_vim(file_path: &PathBuf) -> () {
    Command::new("nvim")
        .arg(file_path)
        .status()
        .expect("Could not open neovim");
}
