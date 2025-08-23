use anyhow::Result;
use colored::Colorize;
use git2::Repository;

pub async fn run() -> Result<()> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("HEAD");
    
    println!("{} {}", "Branch:".bright_blue(), branch.yellow());
    
    let statuses = repo.statuses(None)?;
    if statuses.is_empty() {
        println!("{}", "Working tree clean".green());
        return Ok(());
    }
    
    let mut modified = 0;
    let mut added = 0;
    let mut deleted = 0;
    
    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(git2::Status::WT_MODIFIED) || status.contains(git2::Status::INDEX_MODIFIED) {
            modified += 1;
        }
        if status.contains(git2::Status::WT_NEW) || status.contains(git2::Status::INDEX_NEW) {
            added += 1;
        }
        if status.contains(git2::Status::WT_DELETED) || status.contains(git2::Status::INDEX_DELETED) {
            deleted += 1;
        }
    }
    
    if modified > 0 {
        println!("  {} modified", modified.to_string().yellow());
    }
    if added > 0 {
        println!("  {} added", added.to_string().green());
    }
    if deleted > 0 {
        println!("  {} deleted", deleted.to_string().red());
    }
    
    if let Ok(mut remote) = repo.find_remote("origin") {
        let _ = remote.connect(git2::Direction::Fetch);
        if let Some(head_oid) = head.target() {
            if let Ok(upstream) = repo.find_branch(&format!("origin/{}", branch), git2::BranchType::Remote) {
                if let Some(upstream_oid) = upstream.get().target() {
                    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid)?;
                    if ahead > 0 || behind > 0 {
                        println!("\n{} {} ahead, {} behind", 
                            "Remote:".bright_blue(),
                            ahead.to_string().green(),
                            behind.to_string().yellow()
                        );
                    }
                }
            }
        }
    }
    
    Ok(())
}