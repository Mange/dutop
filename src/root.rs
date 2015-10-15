use std::fmt;
use std::fs;
use std::path::Path;
use std::slice::Iter;

use DisplayableEntry;
use entry::Entry;
use utils;
use utils::SizeDisplay;

pub struct Root {
    name: String,
    entry: Entry,
}

impl Root {
    pub fn for_path(path: &Path) -> Result<Root, String> {
        match fs::metadata(path) {
            Ok(metadata) => Root::from_metadata(path, &metadata),
            Err(error) => Err(utils::describe_io_error(error))
        }
    }

    fn from_metadata(path: &Path, metadata: &fs::Metadata) -> Result<Root, String> {
        Entry::from_metadata(path, &metadata).map(|entry| {
            Root{
                name: utils::full_name_from_path(path, true),
                entry: entry,
            }
        })
    }
}

impl DisplayableEntry for Root {
    type Child = Entry;

    fn size(&self) -> u64 {
        self.entry.size()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn children_iter(&self) -> Iter<Entry> {
        self.entry.children_iter()
    }

    fn is_hidden(&self) -> bool {
        // Roots are never hidden; we always want to show them since the user gave them to us
        // explicitly.
        // Roots can also have the name ".", so they would appear to be hidden in that case unless
        // we handle it differently.
        false
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.entry.size().as_size_display())
    }
}
