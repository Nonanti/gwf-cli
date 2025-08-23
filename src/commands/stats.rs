use anyhow::Result;
use colored::Colorize;
use git2::Repository;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

// TODO: add language stats
// TODO: add contribution graph

pub async fn run() -> Result<()> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    let mut total_commits = 0;
    let mut authors: HashMap<String, usize> = HashMap::new();
    let mut daily_commits: HashMap<String, usize> = HashMap::new();
    let mut file_changes = 0;
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        total_commits += 1;
        
        if let Some(author) = commit.author().name() {
            *authors.entry(author.to_string()).or_insert(0) += 1;
        }
        
        let time = commit.time();
        let dt = DateTime::<Utc>::from_timestamp(time.seconds(), 0)
            .unwrap_or_else(|| Utc::now());
        let date_str = dt.format("%Y-%m-%d").to_string();
        *daily_commits.entry(date_str).or_insert(0) += 1;
        
        if let Ok(tree) = commit.tree() {
            file_changes += tree.len();
        }
    }
    
    println!("{}", "Repository Statistics".bright_blue().bold());
    println!("{}", "─".repeat(30).dimmed());
    
    println!("{} {}", "Total commits:".cyan(), total_commits.to_string().yellow());
    println!("{} {}", "Contributors:".cyan(), authors.len().to_string().yellow());
    
    let mut branches = 0;
    repo.branches(None)?.for_each(|_| { branches += 1; });
    println!("{} {}", "Branches:".cyan(), branches.to_string().yellow());
    
    let mut tags = 0;
    repo.tag_foreach(|_, _| { tags += 1; true })?;
    if tags > 0 {
        println!("{} {}", "Tags:".cyan(), tags.to_string().yellow());
    }
    
    println!("\n{}", "Top Contributors".bright_blue());
    println!("{}", "─".repeat(30).dimmed());
    
    let mut sorted_authors: Vec<_> = authors.iter().collect();
    sorted_authors.sort_by(|a, b| b.1.cmp(a.1));
    
    for (i, (author, count)) in sorted_authors.iter().take(5).enumerate() {
        let bar_length = (*count * 20 / total_commits).max(1);
        let bar = "█".repeat(bar_length);
        println!("{:2}. {:20} {} {}",
            i + 1,
            author.chars().take(20).collect::<String>(),
            bar.green(),
            count.to_string().dimmed()
        );
    }
    
    let now = Utc::now();
    let week_ago = now - Duration::days(7);
    let mut week_commits = 0;
    
    for (date_str, count) in &daily_commits {
        if let Ok(date) = date_str.parse::<DateTime<Utc>>() {
            if date > week_ago {
                week_commits += count;
            }
        }
    }
    
    if week_commits > 0 {
        println!("\n{} {} commits", "Last 7 days:".cyan(), week_commits.to_string().yellow());
    }
    
    Ok(())
}