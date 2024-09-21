pub(crate) const ALIGN: usize = 5;

macro_rules! log {
  ($style:expr, $tag:literal, $fmt:literal$($arg:tt)*) => {
    ::anstream::eprintln!(
      concat!["{_style}{_tag: >_align$}{_style:#} {}"],
      format_args!($fmt$($arg)*),
      _tag = $tag,
      _style = $style,
      _align = $crate::log::ALIGN
    );
  };
}

macro_rules! dbg {
  ($fmt:literal$($arg:tt)*) => {
    $crate::log::log!(
      ::anstyle::Style::new()
        .fg_color(Some(::anstyle::Color::from(::anstyle::AnsiColor::BrightMagenta))),
      "Debug",
      $fmt
      $($arg)*
    )
  };
}

macro_rules! err {
  ($fmt:literal$($arg:tt)*) => {
    $crate::log::log!(
      ::anstyle::Style::new()
        .fg_color(Some(::anstyle::Color::from(::anstyle::AnsiColor::BrightRed))),
      "Error",
      $fmt
      $($arg)*
    )
  };
}

macro_rules! inf {
  ($fmt:literal$($arg:tt)*) => {
    $crate::log::log!(
      ::anstyle::Style::new()
        .fg_color(Some(::anstyle::Color::from(::anstyle::AnsiColor::Cyan))),
      "Info",
      $fmt
      $($arg)*
    )
  };
}

macro_rules! wrn {
  ($fmt:literal$($arg:tt)*) => {
    $crate::log::log!(
      ::anstyle::Style::new()
        .fg_color(Some(::anstyle::Color::from(::anstyle::AnsiColor::Yellow))),
      "Warn",
      $fmt
      $($arg)*
    )
  };
}

pub(crate) use {dbg, err, inf, log, wrn};
