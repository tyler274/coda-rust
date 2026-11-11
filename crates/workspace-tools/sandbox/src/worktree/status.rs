//! `worktree status`: worktrees, their targets and disk use.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use super::walk::newest_below;
use super::{Layout, current_seed};
use crate::error::Error;
use crate::git::git;

/// A UTC calendar date, `YYYY-MM-DD`, for a time.
pub fn utc_date(time: SystemTime) -> String {
    let secs = time
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let days = i64::try_from(secs / 86_400).unwrap_or(i64::MAX);
    // Howard Hinnant's days-to-civil algorithm.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// The output of a reporting tool, or `None` when it is absent or fails;
/// status is a display, and a missing figure shows as `-`.
fn report(program: &str, args: &[&std::ffi::OsStr]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn status(layout: &Layout) -> Result<String, Error> {
    let mut out = String::new();
    // A build root this user cannot read (the host's view of the
    // container's) still shows the worktrees.
    let seed = match current_seed(layout) {
        Ok(Some(seed)) => seed
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().into_owned()),
        Ok(None) => "none".to_owned(),
        Err(error) => format!("unknown ({})", error.chain()),
    };
    out.push_str(&format!("Seed: {seed}\n"));
    out.push_str(&format!(
        "{:<40} {:<40} {:<12} TARGET-ALLOCATED\n",
        "WORKTREE", "BRANCH", "LAST-BUILD"
    ));
    for slug in layout.tree_slugs()? {
        let tree = layout.tree(&slug);
        let dest = layout.target(&slug);
        let branch = git(&tree)
            .args(["symbolic-ref", "-q", "--short", "HEAD"])
            .line_if_ok()?
            .unwrap_or_else(|| "(detached)".to_owned());
        let (last, used) = if dest.is_dir() {
            let last = newest_below(&dest, 3)?.map_or_else(|| "-".to_owned(), utc_date);
            let used = report("du", &["-sh".as_ref(), dest.as_os_str()])
                .and_then(|text| text.split_whitespace().next().map(str::to_owned))
                .unwrap_or_else(|| "-".to_owned());
            (last, used)
        } else {
            ("-".to_owned(), "-".to_owned())
        };
        let slug = slug.as_str();
        out.push_str(&format!("{slug:<40} {branch:<40} {last:<12} {used}\n"));
    }
    out.push_str(
        "Allocated sizes count blocks shared with the seed in full; the dataset totals below do not.\n",
    );
    if let Some(df) = report(
        "df",
        &[
            "-h".as_ref(),
            "--output=target,used,avail".as_ref(),
            layout.build.as_os_str(),
            layout.main.as_os_str(),
        ],
    ) {
        out.push_str(&df);
    }
    let pool = report(
        "df",
        &["--output=source".as_ref(), layout.build.as_os_str()],
    )
    .and_then(|text| text.lines().nth(1).map(str::to_owned))
    .and_then(|source| source.split('/').next().map(str::to_owned));
    if let Some(pool) = pool
        && let Some(saved) = report(
            "zpool",
            &[
                "get".as_ref(),
                "-H".as_ref(),
                "-o".as_ref(),
                "name,property,value".as_ref(),
                "bclonesaved,bcloneratio".as_ref(),
                pool.as_ref(),
            ],
        )
    {
        out.push_str(&saved);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn utc_dates() {
        assert_eq!(utc_date(UNIX_EPOCH), "1970-01-01");
        let leap = UNIX_EPOCH + Duration::from_secs(951_782_400);
        assert_eq!(utc_date(leap), "2000-02-29");
        let later = UNIX_EPOCH + Duration::from_secs(1_794_355_200);
        assert_eq!(utc_date(later), "2026-11-11");
    }
}
