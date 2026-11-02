//! dndevs-board: the live DnDevs board for Dawa's terminal (read-only).
//!
//! It reads /proc, the Forge folder and the RoundTable; it never writes, launches or kills.
//! Every printed line fits 96 columns (the Dawa_96 rule).
//!
//! Usage: dndevs-board [--watch [SECONDS]] [--full] [--json] [--home DIR]

use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime};

const WIDTH: usize = 96;

/// One seat of the board: where it works and which process name runs it.
struct Seat {
    number: u8,
    name: &'static str,
    window: Window,
    account: &'static str,
    dir: &'static str,
    process: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
enum Window {
    Claude,
    OpenCode,
}

const SEATS: [Seat; 5] = [
    Seat {
        number: 1,
        name: "Vex·Lhlava",
        window: Window::Claude,
        account: "dawa.gabond05",
        dir: "Umbra-Keep_Vex-Lhlava",
        process: "claude",
    },
    Seat {
        number: 2,
        name: "Young.Sage",
        window: Window::Claude,
        account: "dawa.gabond05",
        dir: "Young-Sage_02",
        process: "claude",
    },
    Seat {
        number: 3,
        name: "Lumen",
        window: Window::OpenCode,
        account: "dawa.gabond05",
        dir: "Lumen-Castle_Lux-Valhla",
        process: "opencode",
    },
    Seat {
        number: 4,
        name: "Av:vA",
        window: Window::OpenCode,
        account: "copilot",
        dir: "Destiny-Tower_AvvA-Avalhla",
        process: "opencode",
    },
    Seat {
        number: 5,
        name: "Z_71",
        window: Window::OpenCode,
        account: "openai",
        dir: "Avalhla",
        process: "opencode",
    },
];

struct Style {
    on: bool,
}

impl Style {
    fn paint(&self, code: &str, text: &str) -> String {
        if self.on {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }
}

struct Options {
    watch: Option<u64>,
    full: bool,
    json: bool,
    home: PathBuf,
}

fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut o = Options {
        watch: None,
        full: false,
        json: false,
        home: PathBuf::from("/var/home/VVgbon"),
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--watch" => {
                let secs = args.get(i + 1).and_then(|s| s.parse::<u64>().ok());
                if secs.is_some() {
                    i += 1;
                }
                o.watch = Some(secs.unwrap_or(2).clamp(1, 3600));
            }
            "--full" => o.full = true,
            "--json" => o.json = true,
            "--home" => {
                let dir = args.get(i + 1).ok_or("--home needs a folder")?;
                o.home = PathBuf::from(dir);
                i += 1;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }
    Ok(o)
}

/// PIDs whose process name (comm or argv[0] basename) is `name` and whose working folder is
/// `dir`.
fn pids_in(name: &str, dir: &Path) -> Vec<u32> {
    let Ok(entries) = fs::read_dir("/proc") else {
        return Vec::new();
    };
    let me = std::process::id();
    let mut pids: Vec<u32> = entries
        .filter_map(|e| e.ok()?.file_name().to_str()?.parse::<u32>().ok())
        .filter(|&pid| pid != me)
        .filter(|&pid| {
            let base = PathBuf::from(format!("/proc/{pid}"));
            let comm = fs::read_to_string(base.join("comm")).unwrap_or_default();
            let argv0 = fs::read(base.join("cmdline"))
                .ok()
                .and_then(|b| b.split(|&c| c == 0).next().map(|s| s.to_vec()))
                .map(|s| String::from_utf8_lossy(&s).into_owned())
                .unwrap_or_default();
            let argv0_base = argv0.rsplit('/').next().unwrap_or("");
            let named = comm.trim() == name || argv0_base == name;
            named && fs::read_link(base.join("cwd")).is_ok_and(|cwd| cwd == dir)
        })
        .collect();
    pids.sort_unstable();
    pids
}

fn git_short(repo: &Path, rev: &str) -> String {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["rev-parse", "--short", rev])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "?".into())
}

/// The newest file in `dir` whose name starts with `prefix` and ends with `suffix`.
fn newest(dir: &Path, prefix: &str, suffix: &str) -> Option<PathBuf> {
    fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            n.starts_with(prefix) && n.ends_with(suffix)
        })
        .max_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        })
        .map(|e| e.path())
}

fn count_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0)
}

/// Number of issues reported by the strict RoundTable audit, or None if it cannot run.
fn audit_issues(rt: &Path) -> Option<usize> {
    let out = Command::new("python3")
        .arg("roundtable_audit.py")
        .arg("--strict")
        .current_dir(rt)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    Some(text.lines().filter(|l| l.starts_with("  - '")).count())
}

fn clock() -> String {
    Command::new("date")
        .arg("+%m/%d/%y-%I:%M:%S%p %Z")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "UNKNOWN".into())
}

/// A snapshot of everything the board shows.
struct Snapshot {
    clock: String,
    seats: Vec<(u8, &'static str, Window, &'static str, Vec<u32>)>,
    main: String,
    stack: String,
    rehearsal: String,
    tree: String,
    inbox: usize,
    outbox: usize,
    issues: Option<usize>,
    next: String,
}

fn snapshot(home: &Path, issues: Option<usize>) -> Snapshot {
    let forge = home.join("NIGHTGLASS/forge");
    let stack_dir = forge.join("stack");
    let log = newest(&stack_dir, "rehearsal", ".log");
    let log_text = log
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok())
        .unwrap_or_default();
    let rehearsal = if log_text.contains("REHEARSAL PASS") {
        "PASS"
    } else if log_text.contains("REHEARSAL FAILED") {
        "FAIL"
    } else {
        "?"
    };
    let tree = log_text
        .lines()
        .rev()
        .find_map(|l| l.strip_prefix("full_tree="))
        .unwrap_or("?")
        .to_string();
    let stack = newest(&stack_dir, "stack", ".mbox")
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "?".into());
    let rt = home.join("Avalhla/memory/roundtable");
    let next = fs::read_to_string(home.join(".config/dndevs/NEXT.md"))
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        .unwrap_or("(no next action written)")
        .to_string();
    Snapshot {
        clock: clock(),
        seats: SEATS
            .iter()
            .map(|s| {
                let pids = pids_in(s.process, &home.join(s.dir));
                (s.number, s.name, s.window, s.account, pids)
            })
            .collect(),
        main: git_short(&home.join("DnDawa_Binary-Magix"), "main"),
        stack,
        rehearsal: rehearsal.into(),
        tree,
        inbox: count_files(&rt.join("inbox")),
        outbox: count_files(&rt.join("outbox")),
        issues,
        next,
    }
}

/// Cut a line to 96 visible characters (the Dawa_96 rule).
fn fit(line: &str) -> String {
    line.chars().take(WIDTH).collect()
}

fn render(s: &Snapshot, st: &Style, full: bool) -> Vec<String> {
    let mut out = Vec::new();
    out.push(st.paint(
        "1;35",
        &fit(&format!(
            "╔═ DnDevs · {} ═ Dawa decides · ~]{{OK from Dawa only ═╗",
            s.clock
        )),
    ));
    for (label, window) in [("CLAUDE", Window::Claude), ("OPENCODE", Window::OpenCode)] {
        let mut line = format!("{label:<9}");
        let mut painted = line.clone();
        for (n, name, w, _, pids) in &s.seats {
            if *w != window {
                continue;
            }
            let (mark, code) = if pids.is_empty() {
                ("○", "33")
            } else {
                ("●", "32")
            };
            let cell = format!("{mark} {n} {name}");
            line.push_str(&format!("{cell:<20}"));
            let pad = 20usize.saturating_sub(cell.chars().count());
            painted.push_str(&format!("{}{}", st.paint(code, mark), &cell[mark.len()..]));
            painted.push_str(&" ".repeat(pad));
        }
        out.push(if line.chars().count() <= WIDTH {
            painted
        } else {
            fit(&line)
        });
    }
    let verdict = match s.rehearsal.as_str() {
        "PASS" => st.paint("32", "PASS"),
        "FAIL" => st.paint("31", "FAIL"),
        other => other.to_string(),
    };
    out.push(format!(
        "{:<9}main {} · {} rehearsal {} ({})",
        "FORGE", s.main, s.stack, verdict, s.tree
    ));
    let audit = match s.issues {
        Some(0) => st.paint("32", "clean"),
        Some(n) => st.paint("33", &format!("{n} issue(s)")),
        None => "?".into(),
    };
    out.push(format!(
        "{:<9}inbox {} · outbox {} · audit {}",
        "ROUNDTBL", s.inbox, s.outbox, audit
    ));
    let next: String = s.next.chars().take(WIDTH - 9).collect();
    out.push(format!("{:<9}{}", "NEXT", st.paint("36", &next)));
    if full {
        out.push(String::new());
        for (n, name, w, account, pids) in &s.seats {
            let window = if *w == Window::Claude {
                "claude"
            } else {
                "opencode"
            };
            let pids: Vec<String> = pids.iter().map(u32::to_string).collect();
            out.push(fit(&format!(
                "{n}  {name:<11} {window:<9} {account:<14} pid {}",
                if pids.is_empty() {
                    "-".into()
                } else {
                    pids.join(" ")
                }
            )));
        }
    }
    out
}

fn json(s: &Snapshot) -> String {
    let seats: Vec<String> = s
        .seats
        .iter()
        .map(|(n, name, _, account, pids)| {
            let pids: Vec<String> = pids.iter().map(u32::to_string).collect();
            format!(
                "{{\"seat\":{n},\"name\":\"{name}\",\"account\":\"{account}\",\"pids\":[{}]}}",
                pids.join(",")
            )
        })
        .collect();
    let issues = s.issues.map_or("null".to_string(), |n| n.to_string());
    format!(
        "{{\"clock\":\"{}\",\"seats\":[{}],\"main\":\"{}\",\"stack\":\"{}\",\
         \"rehearsal\":\"{}\",\"tree\":\"{}\",\"inbox\":{},\"outbox\":{},\"issues\":{}}}",
        s.clock,
        seats.join(","),
        s.main,
        s.stack,
        s.rehearsal,
        s.tree,
        s.inbox,
        s.outbox,
        issues
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = match parse_args(&args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("dndevs-board: {e}");
            eprintln!("usage: dndevs-board [--watch [SECONDS]] [--full] [--json] [--home DIR]");
            std::process::exit(2);
        }
    };
    let style = Style {
        on: io::stdout().is_terminal() && !opts.json,
    };
    let rt = opts.home.join("Avalhla/memory/roundtable");
    let Some(every) = opts.watch else {
        let snap = snapshot(&opts.home, audit_issues(&rt));
        if opts.json {
            println!("{}", json(&snap));
        } else {
            for line in render(&snap, &style, opts.full) {
                println!("{line}");
            }
        }
        return;
    };
    // Watch mode: redraw in place; the audit (a Python run) refreshes once a minute at most.
    let audit_every = Duration::from_secs(60);
    let mut issues = audit_issues(&rt);
    let mut last_audit = Instant::now();
    let mut stdout = io::stdout();
    loop {
        if last_audit.elapsed() >= audit_every {
            issues = audit_issues(&rt);
            last_audit = Instant::now();
        }
        let snap = snapshot(&opts.home, issues);
        let mut frame = String::from("\x1b[H\x1b[2J");
        for line in render(&snap, &style, opts.full) {
            frame.push_str(&line);
            frame.push('\n');
        }
        frame.push_str(&format!("live, every {every} s · Ctrl-C to stop\n"));
        if stdout
            .write_all(frame.as_bytes())
            .and_then(|_| stdout.flush())
            .is_err()
        {
            return;
        }
        std::thread::sleep(Duration::from_secs(every));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snap() -> Snapshot {
        Snapshot {
            clock: "11/02/26-11:40:00AM EST".into(),
            seats: SEATS
                .iter()
                .map(|s| (s.number, s.name, s.window, s.account, vec![42]))
                .collect(),
            main: "be19cbb".into(),
            stack: "stack11".into(),
            rehearsal: "PASS".into(),
            tree: "b7245cb".into(),
            inbox: 21,
            outbox: 98,
            issues: Some(0),
            next: "x".repeat(200),
        }
    }

    #[test]
    fn every_line_fits_96_columns_plain_and_full() {
        let st = Style { on: false };
        for full in [false, true] {
            for line in render(&snap(), &st, full) {
                assert!(
                    line.chars().count() <= WIDTH,
                    "{} > 96: {line}",
                    line.chars().count()
                );
            }
        }
    }

    #[test]
    fn colors_do_not_change_the_visible_text() {
        let plain: Vec<String> = render(&snap(), &Style { on: false }, true);
        let painted: Vec<String> = render(&snap(), &Style { on: true }, true);
        let strip = |s: &str| {
            let mut out = String::new();
            let mut esc = false;
            for c in s.chars() {
                match (esc, c) {
                    (false, '\x1b') => esc = true,
                    (true, 'm') => esc = false,
                    (true, _) => {}
                    (false, c) => out.push(c),
                }
            }
            out
        };
        for (p, c) in plain.iter().zip(&painted) {
            assert_eq!(strip(c).trim_end(), p.trim_end());
        }
    }

    #[test]
    fn a_seat_without_pids_shows_stopped() {
        let mut s = snap();
        s.seats[3].4.clear();
        let lines = render(&s, &Style { on: false }, false);
        assert!(lines[2].contains("○ 4 Av:vA"));
        assert!(lines[2].contains("● 3 Lumen"));
    }

    #[test]
    fn arguments_are_checked() {
        let a = |v: &[&str]| parse_args(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(a(&["--watch"]).unwrap().watch, Some(2));
        assert_eq!(a(&["--watch", "5"]).unwrap().watch, Some(5));
        assert_eq!(a(&["--watch", "0"]).unwrap().watch, Some(1));
        assert!(a(&["--nope"]).is_err());
        assert!(a(&["--home"]).is_err());
    }

    #[test]
    fn json_is_one_line_with_every_seat() {
        let j = json(&snap());
        assert!(!j.contains('\n'));
        assert_eq!(j.matches("\"seat\":").count(), 5);
        assert!(j.contains("\"issues\":0"));
    }

    #[test]
    fn a_missing_home_gives_unknowns_not_a_crash() {
        let s = snapshot(Path::new("/nonexistent-dndevs"), None);
        assert_eq!(s.main, "?");
        assert_eq!(s.rehearsal, "?");
        assert_eq!(s.inbox, 0);
        assert!(s.seats.iter().all(|seat| seat.4.is_empty()));
    }
}
