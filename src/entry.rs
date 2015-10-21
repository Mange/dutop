use std::fmt;
use std::fs;
use std::path::Path;
use std::slice::Iter;

use DisplayableEntry;
use utils;
use utils::SizeDisplay;

#[derive(Debug)]
pub struct Entry {
    name: String,
    self_size: u64,
    children: Vec<Entry>,
}

impl Entry {
    pub fn for_path(path: &Path) -> Result<Entry, String> {
        match fs::metadata(path) {
            Ok(metadata) => Entry::from_metadata(path, &metadata),
            Err(error) => Err(utils::describe_io_error(error))
        }
    }

    pub fn from_metadata(path: &Path, metadata: &fs::Metadata) -> Result<Entry, String> {
        let mut children = if metadata.is_dir() {
            Entry::in_directory(path)
        } else if metadata.is_file() {
            vec![]
        } else {
            return Err("not a file or directory".to_string());
        };

        children.sort_by(
            // Note: We change the ordering to get in descending order
            |a, b| b.size().cmp(&a.size())
        );

        Ok(Entry {
            name: utils::short_name_from_path(path, metadata.is_dir()),
            children: children,
            self_size: metadata.len()
        })
    }

    fn in_directory(dir: &Path) -> Vec<Entry> {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                read_dir.filter_map(|child| {
                    match child {
                        // TODO: Don't just ignore errors here; we should print them to STDERR and
                        // *then* ignore them.
                        Ok(child) => Entry::for_path(&child.path()).ok(),
                        Err(..) => None,
                    }
                }).collect()
            },
            Err(..) => Vec::new()
        }
    }

    fn descendent_size(&self) -> u64 {
        self.children.iter().map(|child| child.size()).fold(0, |a, n| a + n)
    }
}

impl DisplayableEntry for Entry {
    type Child = Entry;

    fn size(&self) -> u64 {
        self.self_size + self.descendent_size()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn children_iter(&self) -> Iter<Entry> {
        self.children.iter()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.size().as_size_display())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use DisplayableEntry;
    use std::path::Path;

    #[test]
    fn it_can_be_constructed_with_a_path() {
        let pwd = Entry::for_path(Path::new(".")).unwrap();

        assert_eq!(pwd.name(), "./");
        assert!(pwd.children_iter().count() > 0);
        assert!(pwd.size() > 0);
    }

    #[test]
    fn it_is_error_when_constructed_from_missing_path() {
        let missing = Entry::for_path(Path::new("./does-not-exist"));
        assert_eq!(missing.is_err(), true);
        assert_eq!(missing.unwrap_err(), "File not found");
    }

    #[test]
    fn it_is_hidden_when_filename_starts_with_a_dot() {
        let hidden = Entry::for_path(Path::new("./.gitignore")).unwrap();
        let normal = Entry::for_path(Path::new("./LICENSE")).unwrap();

        assert_eq!(hidden.is_hidden(), true);
        assert_eq!(normal.is_hidden(), false);
    }

    #[test]
    fn file_entry_has_no_children() {
        let file = Entry::for_path(Path::new("./LICENSE")).unwrap();
        assert_eq!(file.children_iter().count(), 0);
    }

    #[test]
    fn it_can_be_displayed() {
        use utils::SizeDisplay;

        let file = Entry::for_path(Path::new("./LICENSE")).unwrap();
        assert_eq!(format!("{}", file), format!("LICENSE {}", file.size().as_size_display()));
    }

    #[test]
    fn it_calculates_size_from_children() {
        let entry = Entry::for_path(Path::new(".")).unwrap();
        let children_size = entry
            .children_iter()
            .map(|child| child.size())
            .fold(0, |sum, item| sum + item);

        assert!(entry.size() >= children_size);
    }
}
