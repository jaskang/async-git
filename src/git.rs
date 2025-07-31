use git2::Repository;

#[derive(Clone, Debug)]
pub struct BranchState {
  pub branch: String,
  pub ahead: usize,
  pub behind: usize,

  pub staged: usize,
  pub modified: usize,
  pub deleted: usize,
  pub untracked: usize,
  pub conflict: usize,
}

pub fn get_branch_state(path: &str) -> Result<BranchState, String> {
  // use libgit crate to get the branch state
  let repo =
    Repository::open(path).map_err(|e| "Could not open repository: ".to_string() + e.message())?;
  let head = repo.head();

  if head.is_err() {
    return Ok(BranchState {
      branch: "".to_string(),
      ahead: 0,
      behind: 0,
      staged: 0,
      modified: 0,
      deleted: 0,
      untracked: 0,
      conflict: 0,
    });
  }
  let head = head.unwrap();
  let branch_name = head.shorthand();
  if branch_name.is_none() {
    return Err("Could not get branchname!".to_string());
  }

  let mut branch_name = branch_name.unwrap().to_string();
  if head.is_tag() {
    let tag = head.peel_to_tag();
    if let Ok(tag) = tag {
      branch_name = tag.name().unwrap().to_string();
    }
    branch_name += " is tag!";
  }

  let branch_name = branch_name;
  let mut ahead = 0;
  let mut behind = 0;

  // get git branch
  if head.is_branch() {
    let branch_name = head.shorthand().unwrap().to_string();
    let branch = repo
      .find_branch(&branch_name, git2::BranchType::Local)
      .map_err(|e| e.message().to_string() + "branch!" + &branch_name)?;
    if let Ok(upstream) = branch.upstream() {
      (ahead, behind) = repo
        .graph_ahead_behind(
          branch.get().target().unwrap(),
          upstream.get().target().unwrap(),
        )
        .map_err(|e| e.message().to_string() + "ahead-behind!")?;
    }
  }
  let (ahead, behind) = (ahead, behind);

  let statuses = repo
    .statuses(None)
    .map_err(|e| e.message().to_string() + "statuses")?;

  let mut staged = 0;
  let mut modified = 0;
  let mut deleted = 0;
  let mut untracked = 0;
  let mut conflict = 0;

  for entry in statuses.iter() {
    let status = entry.status();
    if status.is_index_new()
      || status.is_index_modified()
      || status.is_index_deleted()
      || status.is_index_renamed()
      || status.is_index_typechange()
    {
      staged += 1;
    }
    if status.is_wt_new() {
      untracked += 1;
    }
    if status.is_wt_modified() {
      modified += 1;
    }
    if status.is_wt_deleted() {
      deleted += 1;
    }
    if status.is_conflicted() {
      conflict += 1;
    }
  }

  Ok(BranchState {
    branch: branch_name,
    ahead,
    behind,
    staged,
    modified,
    deleted,
    untracked,
    conflict,
  })
}
