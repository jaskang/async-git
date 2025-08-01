#![deny(clippy::all)]

mod git;
use futures::future::join_all;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::{sync::Semaphore, task, time::Instant};

use crate::git::BranchStatus;

// 带并发限制的版本，避免同时启动过多进程
#[napi]
pub async fn git_status(
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

#[napi]
pub async fn checkout(
  dirs: Vec<String>,
  branch: String,
  limit: Option<u32>,
) -> Result<Vec<Result<String>>> {
  let limit = limit.unwrap_or(10) as usize;
  let semaphore = Arc::new(Semaphore::new(limit));

  let tasks: Vec<_> = dirs
    .into_iter()
    .enumerate()
    .map(|(index, dir)| {
      let sem = semaphore.clone();
      let branch = branch.clone();
      task::spawn(async move {
        let _permit = sem.acquire().await.unwrap();

        let start_time = Instant::now();
        let start_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!(
          "[{}] git checkout {branch} in {dir} start at: {}",
          index, start_timestamp
        );

        let repo = match git2::Repository::open(&dir) {
          Ok(repo) => repo,
          Err(e) => {
            return Err(Error::from_reason(e.message()));
          }
        };

        let checkout_result = git::checkout(&repo, &branch);

        let end_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!(
          "[{}] git checkout {branch} in {dir} end at: {}, took: {:?}",
          index,
          end_timestamp,
          start_time.elapsed()
        );

        checkout_result.map_err(|e| Error::from_reason(e))
      })
    })
    .collect();

  let results = join_all(tasks).await;
  let results = results.into_iter().map(|r| r.unwrap()).collect();

  Ok(results)
}

#[napi(object)]
#[derive(Debug)]
pub struct CloneItem {
  pub url: String,
  pub path: String,
  pub branch: Option<String>,
}

#[napi]
pub async fn clone(repos: Vec<CloneItem>, limit: Option<u32>) -> Result<Vec<Result<String>>> {
  let limit = limit.unwrap_or(10) as usize;
  let semaphore = Arc::new(Semaphore::new(limit));

  let tasks: Vec<_> = repos
    .into_iter()
    .enumerate()
    .map(|(index, repo_info)| {
      let sem = semaphore.clone();
      task::spawn(async move {
        let _permit = sem.acquire().await.unwrap();

        let start_time = Instant::now();
        let start_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!(
          "[{}] git clone {} to {} start at: {}",
          index, repo_info.url, repo_info.path, start_timestamp
        );

        let clone_result = git::clone(&repo_info.url, &repo_info.path, repo_info.branch.as_deref());

        let end_timestamp = chrono::Local::now()
          .format("%Y-%m-%d %H:%M:%S%.3f")
          .to_string();
        println!(
          "[{}] git clone {} to {} end at: {}, took: {:?}",
          index,
          repo_info.url,
          repo_info.path,
          end_timestamp,
          start_time.elapsed()
        );

        clone_result
          .map(|_| {
            format!(
              "Successfully cloned {} to {}",
              repo_info.url, repo_info.path
            )
          })
          .map_err(|e| Error::from_reason(e))
      })
    })
    .collect();

  let results = join_all(tasks).await;
  let results = results.into_iter().map(|r| r.unwrap()).collect();

  Ok(results)
}
