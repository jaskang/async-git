#![deny(clippy::all)]

mod git;
use futures::stream::{iter, StreamExt};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;
use tokio::{process::Command, time::Instant};
#[napi(object)]
pub struct GitStatus {
  pub root: String,
  pub not_added: Vec<String>,
  pub conflicted: Vec<String>,
  pub created: Vec<String>,
  pub deleted: Vec<String>,
  pub ignored: Vec<String>,
  pub modified: Vec<String>,
  pub renamed: Vec<String>,
  pub files: Vec<String>,
  pub staged: Vec<String>,
  pub ahead: u32,
  pub behind: u32,
  pub current: Option<String>,
  pub tracking: Option<String>,
  pub detached: bool,
  pub is_clean: bool,
}

// ## feat/memory...origin/feat/memory
//  M app/base/build/ws.app.js
//  M app/pages/misc/asyncWecareComponent
//  M app/pages/misc/claimWxAssistant
//  M app/pages/misc/up
//  M app/pages/misc/wecare
// ?? TABLE.md
// ?? TOC.md
// ?? TODO.md
// ?? cli.config.js
// ?? modules.new.json
pub fn parse_status(root: String, output: String) -> GitStatus {
  let start_time = Instant::now();
  let _ = git::get_branch_state(&root);
  println!("git2 start at: {:?}", start_time.elapsed());
  let mut status = GitStatus {
    root,
    not_added: Vec::new(),
    conflicted: Vec::new(),
    created: Vec::new(),
    deleted: Vec::new(),
    ignored: Vec::new(),
    modified: Vec::new(),
    renamed: Vec::new(),
    files: Vec::new(),
    staged: Vec::new(),
    ahead: 0,
    behind: 0,
    current: None,
    tracking: None,
    detached: false,
    is_clean: true,
  };

  let lines: Vec<&str> = output.split('\0').collect();

  // Parse branch info from first line
  if let Some(branch_line) = lines.first() {
    if branch_line.starts_with("##") {
      let branch_info = &branch_line[3..];

      // Parse current and tracking branch
      if let Some((current, tracking)) = branch_info.split_once("...") {
        status.current = Some(current.trim().to_string());

        if let Some(tracking_info) = tracking.split_once(' ') {
          status.tracking = Some(tracking_info.0.to_string());

          // Parse ahead/behind info
          if tracking_info.1.contains("ahead") {
            if let Some(num) = tracking_info
              .1
              .split_whitespace()
              .find(|s| s.parse::<u32>().is_ok())
            {
              status.ahead = num.parse().unwrap_or(0);
            }
          }
          if tracking_info.1.contains("behind") {
            if let Some(num) = tracking_info
              .1
              .split_whitespace()
              .find(|s| s.parse::<u32>().is_ok())
            {
              status.behind = num.parse().unwrap_or(0);
            }
          }
        }
      } else {
        status.current = Some(branch_info.trim().to_string());
        status.detached = true;
      }
    }
  }

  // Parse file statuses
  for line in lines.iter().skip(1) {
    if line.is_empty() {
      continue;
    }

    let (status_code, file_path) = line.split_at(2);
    let file_path = file_path.trim();
    status.files.push(file_path.to_string());

    match status_code {
      "??" => status.not_added.push(file_path.to_string()),
      "M " => status.modified.push(file_path.to_string()),
      "A " => status.created.push(file_path.to_string()),
      "D " => status.deleted.push(file_path.to_string()),
      "R " => status.renamed.push(file_path.to_string()),
      "U " => status.modified.push(file_path.to_string()),
      " D" => status.deleted.push(file_path.to_string()),
      "UU" => status.conflicted.push(file_path.to_string()),
      _ => {}
    }
  }

  status.is_clean = status.files.is_empty();
  status
}

// 带并发限制的版本，避免同时启动过多进程
#[napi]
pub async fn git_status(dirs: Vec<String>, limit: Option<u32>) -> Result<Vec<Result<GitStatus>>> {
  // 记录开始时间
  let start_time = Instant::now();

  println!("git status start at: {:?}", start_time);

  let limit = limit.unwrap_or(10) as usize;
  let results = iter(dirs)
    .map(|dir| async move {
      // 记录开始时间
      let start_time = Instant::now();
      let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("-b")
        .arg("-u")
        .arg("--null")
        .current_dir(&dir)
        .output()
        .await;

      println!("git status took: {:?}", start_time.elapsed());
      match output {
        Ok(output) => {
          if output.status.success() {
            Ok(parse_status(
              dir,
              String::from_utf8_lossy(&output.stdout).to_string(),
            ))
          } else {
            Err(Error::from_reason(String::from_utf8_lossy(&output.stderr)))
          }
        }
        Err(e) => Err(Error::from_reason(e.to_string())),
      }
    })
    .buffer_unordered(limit)
    .collect::<Vec<_>>()
    .await;

  Ok(results)
}

// 带并发限制的版本，避免同时启动过多进程
#[napi]
pub async fn git_status_with_fetch(
  dirs: Vec<String>,
  limit: Option<u32>,
) -> Result<Vec<Result<GitStatus>>> {
  let limit = limit.unwrap_or(10) as usize;

  let results = iter(dirs)
    .map(|dir| async move {
      // 记录开始时间
      let start_time = Instant::now();

      // 先尝试获取远程分支信息
      // 这里的目的是为了确保在执行 git status 时，能够获取到最新的远程分支状态
      // 如果报错了，需要忽略 直接进行下一步 git status
      let _ = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg("sit")
        .current_dir(&dir)
        .output()
        .await;

      let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("-b")
        .arg("-u")
        .arg("--null")
        .current_dir(&dir)
        .output()
        .await;

      println!("git status took: {:?}", start_time.elapsed());
      match output {
        Ok(output) => {
          if output.status.success() {
            Ok(parse_status(
              dir,
              String::from_utf8_lossy(&output.stdout).to_string(),
            ))
          } else {
            Err(Error::from_reason(String::from_utf8_lossy(&output.stderr)))
          }
        }
        Err(e) => Err(Error::from_reason(e.to_string())),
      }
    })
    .buffer_unordered(limit)
    .collect::<Vec<_>>()
    .await;

  Ok(results)
}
