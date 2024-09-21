mod log;
mod unstable;

use std::{
  fmt,
  path::{Path, PathBuf},
  process,
};

use log::{err, inf, wrn};
use unstable::partition_dedup_by;

const SUPPORTED_FILE_EXTS: &[&str] = &["css"];

#[derive(clap::Parser, Debug)]
struct Command {
  #[clap(short, long)]
  verbose: bool,
  #[clap(short, long)]
  watch: bool,
  paths: Vec<PathBuf>,
}

struct Context {
  verbose: bool,
}

impl Context {
  fn info(&self, args: fmt::Arguments) {
    if self.verbose {
      inf!("{}", args);
    }
  }
}

fn file(ctx: &Context, path: &Path) {
  if !path.extension().map_or(false, |file_ext| {
    SUPPORTED_FILE_EXTS
      .iter()
      .any(|supported| file_ext.eq_ignore_ascii_case(supported))
  }) {
    ctx.info(format_args!("Ignoring {path:?}: unsupported extension"));
    return;
  }

  todo!("{path:?}")
}

fn dir(ctx: &Context, path: &Path) {
  ctx.info(format_args!("Reading directory {path:?}"));
  let entries = match path.read_dir() {
    Ok(v) => v,
    Err(error) => {
      err!("Failed to read directory {path:?}: {error}");
      process::exit(1); // TODO: Should it exit, or ignore and go on?
    }
  };

  for entry in entries {
    let entry = match entry {
      Ok(v) => v,
      Err(error) => {
        err!("Failed to read directory {path:?}: {error}");
        process::exit(1); // TODO: Should it exit, or ignore and go on?
      }
    };
    generic(ctx, &entry.path());
  }
}

fn generic(ctx: &Context, path: &Path) {
  if !path.exists() {
    wrn!("{path:?} does not exist");
    return;
  }
  if path.is_dir() {
    return dir(ctx, path);
  }
  if path.is_file() {
    file(ctx, path);
  }
}

fn main() {
  let mut command = <Command as clap::Parser>::parse();

  if command.verbose {
    wrn!("Verbose mode is not implemented yet");
  }

  if command.watch {
    wrn!("Watch mode is not implemented yet");
  }

  let (paths, dups) = partition_dedup_by(&mut command.paths, |a, b| a == b);
  for dup in dups {
    wrn!("{dup:?} is a duplicate");
  }

  let ctx = Context {
    verbose: command.verbose,
  };
  for path in paths {
    generic(&ctx, path);
  }
}
