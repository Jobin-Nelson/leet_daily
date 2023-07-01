use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_daily_qn_link() -> Result<(String, String), reqwest::Error> {
    let query_url = "https://leetcode.com/graphql";

    let query = HashMap::from([
        ("query",  "query questionOfToday {\n\tactiveDailyCodingChallengeQuestion {\n\t\tdate\n\t\tlink\n\t}\n}\n"),
        ("operationName", "questionOfToday")
    ]);

    let client = reqwest::blocking::Client::new();
    let data: serde_json::Value = client.post(query_url).json(&query).send()?.json()?;

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

pub fn open_link_in_browser(daily_qn_link: &String) -> () {
    Command::new("firefox")
        .arg(daily_qn_link)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Could not open link in brave browser");
}

pub fn create_file_path(daily_qn_link: &String) -> PathBuf {
    let now = chrono::Utc::now();

    let year = now.format("%Y").to_string();

    let month = now.format("%B").to_string().to_ascii_lowercase();

    let mut file_path: PathBuf = [
        "/home/jobin/playground/projects/learn/competitive_programming",
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

pub fn fetch_content(daily_qn_link: &String) -> Option<String> {
    todo!();
}

pub fn create_file(file_path: &PathBuf, current_date: &String, daily_qn_link: &String) -> () {
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

pub fn open_file_in_vim(file_path: &PathBuf) -> () {
    Command::new("nvim")
        .arg(file_path)
        .status()
        .expect("Could not open neovim");
}
