use std::fmt;
use std::fs;
use std::path::Path;
use std::slice::Iter;

use modes::DisplayableEntry;
use entry::Entry;
use utils;
use utils::SizeDisplay;

#[derive(Debug)]
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
                name: utils::full_name_from_path(path, metadata.is_dir()),
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

#[cfg(test)]
mod test {
    use super::*;
    use modes::DisplayableEntry;
    use std::path::Path;

    #[test]
    fn it_can_be_constructed_with_a_path() {
        let pwd = Root::for_path(Path::new(".")).unwrap();

        assert_eq!(pwd.name(), "./");
        assert_eq!(pwd.is_hidden(), false);
        assert!(pwd.children_iter().count() > 0);
        assert!(pwd.size() > 0);
    }

    #[test]
    fn it_is_error_when_constructed_from_missing_path() {
        let missing = Root::for_path(Path::new("./does-not-exist"));
        assert_eq!(missing.is_err(), true);
        assert_eq!(missing.unwrap_err(), "File not found");
    }

    #[test]
    fn it_can_be_displayed() {
        use utils::SizeDisplay;

        let root = Root::for_path(Path::new("./LICENSE")).unwrap();
        assert_eq!(format!("{}", root), format!("./LICENSE {}", root.size().as_size_display()));
    }

    #[test]
    fn it_calculates_size_from_children() {
        let root = Root::for_path(Path::new(".")).unwrap();
        let children_size = root
            .children_iter()
            .map(|child| child.size())
            .fold(0, |sum, item| sum + item);

        assert!(root.size() >= children_size);
    }
}
