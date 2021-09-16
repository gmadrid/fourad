use std::sync::Mutex;

// TODO: move this to a separate crate for reusability.

static INSTANCE: once_cell::sync::Lazy<Mutex<Spew>> = once_cell::sync::Lazy::new(|| {
    Mutex::new(Spew {
        level: SpewLevel::STANDARD,
    })
});

#[macro_export]
macro_rules! quiet {
    ($($arg:tt)+) => ($crate::quiet_fa(format_args!($($arg)+)));
}

#[macro_export]
macro_rules! spew {
    ($($arg:tt)+) => ($crate::spew_fa(format_args!($($arg)+)));
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => ($crate::verbose_fa(format_args!($($arg)+)));
}

pub fn spew_fa(fa: std::fmt::Arguments) {
    spew_(std::fmt::format(fa))
}

fn spew_(s: impl AsRef<str>) {
    INSTANCE
        .lock()
        .unwrap()
        .spew_at_level(SpewLevel::STANDARD, s)
}

pub fn quiet_fa(fa: std::fmt::Arguments) {
    quiet_(std::fmt::format(fa))
}

fn quiet_(s: impl AsRef<str>) {
    INSTANCE.lock().unwrap().spew_at_level(SpewLevel::QUIET, s)
}

pub fn verbose_fa(fa: std::fmt::Arguments) {
    verbose_(std::fmt::format(fa))
}

fn verbose_(s: impl AsRef<str>) {
    INSTANCE
        .lock()
        .unwrap()
        .spew_at_level(SpewLevel::VERBOSE, s)
}

pub fn set_level(level: SpewLevel) {
    INSTANCE.lock().unwrap().set_level(level)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SpewLevel {
    QUIET,
    STANDARD,
    VERBOSE,
}

struct Spew {
    level: SpewLevel,
}

impl Spew {
    fn set_level(&mut self, level: SpewLevel) {
        self.level = level
    }

    fn spew_at_level(&self, level: SpewLevel, s: impl AsRef<str>) {
        if level <= self.level {
            println!("{}", s.as_ref());
        }
    }
}
