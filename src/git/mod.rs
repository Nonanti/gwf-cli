use anyhow::{bail, Context, Result};
use git2::{BranchType, Repository};
use std::path::Path;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open_current() -> Result<Self> {
        let repo = Repository::open_from_env()
            .or_else(|_| Repository::discover("."))
            .context("Failed to open git repository")?;

        Ok(Self { repo })
    }

    #[allow(dead_code)]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open git repository")?;

        Ok(Self { repo })
    }

    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head().context("Failed to get HEAD reference")?;

        if !head.is_branch() {
            bail!("HEAD is not pointing to a branch (detached HEAD state)");
        }

        let branch_name = head.shorthand().context("Failed to get branch name")?;

        Ok(branch_name.to_string())
    }

    pub fn list_remotes(&self) -> Result<Vec<String>> {
        let remotes = self.repo.remotes().context("Failed to list remotes")?;

        let remote_names: Vec<String> = remotes
            .iter()
            .filter_map(|name| name.map(String::from))
            .collect();

        Ok(remote_names)
    }

    pub fn create_branch(&self, name: &str, from: Option<&str>) -> Result<()> {
        let target = if let Some(from_branch) = from {
            let from_ref = self
                .repo
                .find_branch(from_branch, BranchType::Local)
                .context(format!("Branch '{}' not found", from_branch))?;
            from_ref
                .get()
                .target()
                .context("Failed to get branch target")?
        } else {
            self.repo
                .head()?
                .target()
                .context("Failed to get HEAD target")?
        };

        let commit = self
            .repo
            .find_commit(target)
            .context("Failed to find commit")?;

        self.repo
            .branch(name, &commit, false)
            .context(format!("Failed to create branch '{}'", name))?;

        Ok(())
    }

    pub fn checkout(&self, branch_name: &str) -> Result<()> {
        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{}", branch_name))
            .context(format!("Failed to find branch '{}'", branch_name))?;

        self.repo
            .checkout_tree(&obj, None)
            .context("Failed to checkout tree")?;

        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .context("Failed to update HEAD")?;

        Ok(())
    }

    pub fn has_uncommitted_changes(&self) -> Result<bool> {
        let statuses = self
            .repo
            .statuses(None)
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    pub fn list_branches(&self, include_remote: bool) -> Result<Vec<String>> {
        let mut branches = Vec::new();

        let branch_type = if include_remote {
            BranchType::Remote
        } else {
            BranchType::Local
        };

        for branch in self.repo.branches(Some(branch_type))? {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branches.push(name.to_string());
            }
        }

        Ok(branches)
    }

    pub fn is_branch_merged(&self, branch_name: &str, into: &str) -> Result<bool> {
        let branch = self
            .repo
            .find_branch(branch_name, BranchType::Local)
            .context(format!("Branch '{}' not found", branch_name))?;

        let branch_oid = branch
            .get()
            .target()
            .context("Failed to get branch target")?;

        let into_branch = self
            .repo
            .find_branch(into, BranchType::Local)
            .context(format!("Branch '{}' not found", into))?;

        let into_oid = into_branch
            .get()
            .target()
            .context("Failed to get target branch target")?;

        let base = self
            .repo
            .merge_base(branch_oid, into_oid)
            .context("Failed to find merge base")?;

        Ok(base == branch_oid)
    }

    pub fn delete_branch(&self, branch_name: &str) -> Result<()> {
        let mut branch = self
            .repo
            .find_branch(branch_name, BranchType::Local)
            .context(format!("Branch '{}' not found", branch_name))?;

        branch
            .delete()
            .context(format!("Failed to delete branch '{}'", branch_name))?;

        Ok(())
    }

    pub fn fetch(&self, remote: &str) -> Result<()> {
        let mut remote = self
            .repo
            .find_remote(remote)
            .context(format!("Remote '{}' not found", remote))?;

        remote
            .fetch(&[] as &[&str], None, None)
            .context("Failed to fetch from remote")?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn pull(&self, remote: &str, branch: &str) -> Result<()> {
        self.fetch(remote)?;

        let fetch_head = self
            .repo
            .find_reference("FETCH_HEAD")
            .context("Failed to find FETCH_HEAD")?;

        let fetch_commit = self
            .repo
            .reference_to_annotated_commit(&fetch_head)
            .context("Failed to get fetch commit")?;

        let analysis = self
            .repo
            .merge_analysis(&[&fetch_commit])
            .context("Failed to analyze merge")?;

        if analysis.0.is_up_to_date() {
            println!("Already up to date");
        } else if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo
                .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            bail!("Merge required - not implemented in this example");
        }

        Ok(())
    }
}
