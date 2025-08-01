use git2::{Branch, Repository, Status};
use napi::Error;
use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Default)]
pub struct BranchStatus {
  pub root: String,   // 仓库根目录
  pub branch: String, // 当前分支名
  pub ahead: u32,     // 领先远程分支的提交数
  pub behind: u32,    // 落后远程分支的提交数

  pub staged: u32,
  pub conflicted: u32,
  pub changed: u32,
  pub untracked: u32,
}

fn ahead_behind(repo: &Repository) -> (u32, u32) {
  let default = (0, 0);

  let head = match repo.head() {
    Ok(head) => Some(head).unwrap(),
    Err(_) => return default,
  };
  let local_oid = head.target().expect("Unable to determine Oid of head.");

  let upstream_branch = Branch::wrap(head);
  let upstream = match upstream_branch.upstream() {
    Ok(u) => u,
    Err(_) => return default,
  };
  let upstream_oid = match upstream.into_reference().target() {
    Some(u) => u,
    None => return default,
  };

  match repo.graph_ahead_behind(local_oid, upstream_oid) {
    Ok(ab) => (ab.0 as u32, ab.1 as u32),
    Err(_) => default,
  }
}

fn get_branch_name(repo: &Repository) -> String {
  let default = String::from("master");

  let head = match repo.head() {
    Ok(head) => head,
    Err(_) => match repo.find_reference("HEAD") {
      Ok(h) => h,
      Err(_) => return default,
    },
  };

  if head.is_branch() {
    // easy case: a checked out branch, give us the name of that branch
    return String::from(
      Branch::wrap(head)
        .name()
        .expect("Unable to determine name of branch.")
        .unwrap(),
    );
  }

  let config = repo
    .config()
    .expect("Unable to open config for this repository.");
  let hash_length = match config.get_i32("core.abbrev") {
    Ok(l) => l + 1,
    Err(_) => 9,
  };

  match head.symbolic_target() {
    // this is an unborn branch probably? and/or like a repo with no
    // commits? so say it's master. who knows man git is weird
    Some(_) => default,
    // this is anything else, generally a specific commit i guess?
    // like `git checkout HEAD~1`
    None => {
      let mut commit = format!(":{}", head.target().unwrap());
      commit.truncate(hash_length as usize);
      commit
    }
  }
}

pub fn fetch(repo: &Repository) -> Result<(), Error> {
  let branch = get_branch_name(repo);
  // Fetch the latest changes from the remote repository
  if let Err(e) = repo
    .find_remote("origin")
    .and_then(|mut remote| remote.fetch(&[branch], None, None))
  {
    return Err(Error::from_reason(format!("Failed to fetch: {}", e)));
  }
  Ok(())
}

pub fn status(repo: &Repository, root: &str) -> Result<BranchStatus, Error> {
  let mut branch_status = BranchStatus {
    root: root.to_string(),
    ..Default::default()
  };

  let (ahead, behind) = ahead_behind(&repo);
  let branch_name = get_branch_name(&repo);

  branch_status.branch = branch_name;
  branch_status.ahead = ahead;
  branch_status.behind = behind;

  let mut status_options = git2::StatusOptions::new();
  status_options.include_untracked(true);
  status_options.renames_from_rewrites(true); // 开启重命名检测
  status_options.renames_head_to_index(true); // 开启从 HEAD 到索引的重命名检测
  status_options.renames_index_to_workdir(true); // 开启从索引到工作区的重命名检测

  let statuses = repo
    .statuses(Some(&mut status_options))
    .expect("Unable to gather status information.");

  let mut staged = Status::empty();
  staged.insert(Status::INDEX_NEW);
  staged.insert(Status::INDEX_MODIFIED);
  staged.insert(Status::INDEX_DELETED);
  staged.insert(Status::INDEX_RENAMED);
  staged.insert(Status::INDEX_TYPECHANGE);

  let mut changed = Status::empty();
  changed.insert(Status::WT_MODIFIED);
  changed.insert(Status::WT_DELETED);
  changed.insert(Status::WT_RENAMED);
  changed.insert(Status::WT_TYPECHANGE);

  for entry in statuses.iter() {
    match entry.status() {
      s if s.intersects(staged) => branch_status.staged += 1,
      s if s.intersects(changed) => branch_status.changed += 1,
      s if s.contains(Status::CONFLICTED) => branch_status.conflicted += 1,
      s if s.contains(Status::WT_NEW) => branch_status.untracked += 1,
      _ => (),
    }
  }

  Ok(branch_status)
}

pub fn clone(url: &str, path: &str, branch: Option<&str>) -> Result<Repository, String> {
  // 检查目标路径是否存在
  if std::path::Path::new(path).exists() {
    return Err(format!("Path '{}' already exists", path));
  }

  let repo = if let Some(branch) = branch {
    // 使用 RepoBuilder 直接指定分支克隆
    let mut builder = git2::build::RepoBuilder::new();
    builder.branch(branch);
    builder
      .clone(url, std::path::Path::new(path))
      .map_err(|e| e.message().to_string())?
  } else {
    // 默认克隆
    Repository::clone(url, path).map_err(|e| e.message().to_string())?
  };

  Ok(repo)
}

/**
 * 切换到指定分支
 * 如果分支不存在，则尝试从远程分支创建
 * 如果远程分支也不存在，则返回错误
 */
pub fn checkout(repo: &Repository, branch: &str) -> Result<String, String> {
  // 尝试本地分支
  if let Ok(local_branch) = repo.find_branch(branch, git2::BranchType::Local) {
    let reference = local_branch.get();
    repo
      .set_head(reference.name().unwrap())
      .map_err(|e| e.message().to_string())?;
    repo
      .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
      .map_err(|e| e.message().to_string())?;
    return Ok(branch.to_string());
  }

  // 尝试从远程分支创建并切换
  let remote_branch_name = format!("origin/{}", branch);
  if let Ok(remote_branch) = repo.find_branch(&remote_branch_name, git2::BranchType::Remote) {
    let commit = remote_branch
      .get()
      .peel_to_commit()
      .map_err(|e| e.message().to_string())?;

    // 创建本地分支
    let mut local_branch = repo
      .branch(branch, &commit, false)
      .map_err(|e| e.message().to_string())?;

    // 设置上游分支
    local_branch
      .set_upstream(Some(&remote_branch_name))
      .map_err(|e| e.message().to_string())?;

    // 切换到新创建的分支
    let reference = local_branch.get();
    repo
      .set_head(reference.name().unwrap())
      .map_err(|e| e.message().to_string())?;
    repo
      .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
      .map_err(|e| e.message().to_string())?;

    return Ok(branch.to_string());
  }

  Err(format!("Branch '{}' not found locally or remotely", branch))
}
