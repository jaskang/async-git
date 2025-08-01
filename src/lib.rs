#![deny(clippy::all)]

mod git;
use futures::{
  future::join_all,
  stream::{iter, StreamExt},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::{path::Path, sync::Arc};
use tokio::{process::Command, sync::Semaphore, task, time::Instant};

use crate::git::BranchStatus;

// 带并发限制的版本，避免同时启动过多进程
#[napi]
pub async fn git_status(
  dirs: Vec<String>,
  limit: Option<u32>,
) -> Result<Vec<Result<BranchStatus>>> {
  // 记录开始时间
  let start_time = Instant::now();

  println!("git status start at: {:?}", start_time);

  let limit = limit.unwrap_or(10) as usize;
  let results = iter(dirs)
    .map(|dir| async move {
      // 记录开始时间
      let start_time = Instant::now();
      println!("git status {dir} start at: {:?}", start_time);
      let repo = match git2::Repository::open(&dir) {
        Ok(repo) => repo,
        Err(e) => {
          return Err(Error::from_reason(e.message()));
        }
      };
      let git2_status = git::status(&repo, &dir);
      println!("git status {dir} took: {:?}", start_time.elapsed());
      git2_status
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
) -> Result<Vec<Result<BranchStatus>>> {
  let limit = limit.unwrap_or(10) as usize;
  let semaphore = Arc::new(Semaphore::new(limit));

  let tasks: Vec<_> = dirs
    .into_iter()
    .enumerate()
    .map(|(index, dir)| {
      let sem = semaphore.clone();
      task::spawn(async move {
        let _permit = sem.acquire().await.unwrap();

        let start_time = Instant::now();
        let start_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!("[{}] git status {dir} start at: {}", index, start_timestamp);

        let repo = match git2::Repository::open(&dir) {
          Ok(repo) => repo,
          Err(e) => {
            return Err(Error::from_reason(e.message()));
          }
        };

        let _ = git::fetch(&repo);
        let git2_status = git::status(&repo, &dir);

        let end_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!(
          "[{}] git status {dir} end at: {}, took: {:?}",
          index,
          end_timestamp,
          start_time.elapsed()
        );

        git2_status
      })
    })
    .collect();

  let results = join_all(tasks).await;
  let results = results.into_iter().map(|r| r.unwrap()).collect();

  Ok(results)
}
