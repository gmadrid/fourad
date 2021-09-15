use std::sync::Mutex;

static INSTANCE: once_cell::sync::Lazy<Mutex<Spew>> = once_cell::sync::Lazy::new(|| {
    Mutex::new(Spew {
        level: SpewLevel::STANDARD,
    })
});

// TODO: make these macros, so that we can forward the format specs.
pub fn spew(s: impl AsRef<str>) {
    INSTANCE
        .lock()
        .unwrap()
        .spew_at_level(SpewLevel::STANDARD, s)
}

pub fn quiet(s: impl AsRef<str>) {
    INSTANCE.lock().unwrap().spew_at_level(SpewLevel::QUIET, s)
}

pub fn verbose(s: impl AsRef<str>) {
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
