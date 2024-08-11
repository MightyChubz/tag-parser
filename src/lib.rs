use std::path::Path;

/// A group is a collection of tags that are related to each other.
///
/// # Example
///
/// ```
/// use tag_parser::Group;
///
/// let group = Group {
///    name: "Generic".to_string(),
///    tags: vec!["red".to_string(), "hair".to_string()],
/// };
///
/// assert_eq!(group.name, "Generic");
/// assert_eq!(group.tags.len(), 2);
/// assert_eq!(group.tags[0], "red");
/// assert_eq!(group.tags[1], "hair");
/// ```
#[derive(Debug)]
pub struct Group {
    /// The name of the group.
    pub name: String,
    /// A list of tags that belong to the group.
    pub tags: Vec<String>,
}

/// A parser that reads a file and extracts groups of tags.
///
/// The file should be in the following format:
/// ```text
/// [Group1]
/// tag1 tag2 tag3
/// tag4 tag5
///
/// [Group2]
/// tag6 tag7
/// ```
///
/// The parser will extract the groups and tags from the file.
///
/// # Example
///
/// ```no_run
/// use tag_parser::TagParser;
/// use std::path::Path;
/// let path = Path::new("tags.txt");
/// let mut parser = TagParser::new(path);
/// parser.parse();
/// let groups = parser.groups();
/// ```
///
/// The `groups` variable will contain a list of groups with their respective tags.
///
/// The `parse` method must be called before accessing the groups.
///
/// The `TagParser` struct also implements the `From<&str>` and `From<String>` traits.
///
/// # Example
///
/// ```
/// use tag_parser::TagParser;
/// let data = include_str!("test_data.txt");
/// let parser = TagParser::from(data);
/// let groups = parser.groups();
/// ```
#[derive(Debug)]
pub struct TagParser {
    /// The data read from the file.
    data: String,
    /// A list of groups extracted from the data.
    groups: Vec<Group>,
}

impl TagParser {
    /// Creates a new `TagParser` instance from a file.
    ///
    /// # Arguments
    ///
    /// * `path`: A path to the file containing the tags.
    ///
    /// returns: A new `TagParser` instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tag_parser::TagParser;
    /// use std::path::Path;
    ///
    /// let path = Path::new("tags.txt");
    /// let parser = TagParser::new(path);
    /// ```
    pub fn new(path: &Path) -> Self {
        let data = std::fs::read_to_string(path).unwrap();
        Self {
            data,
            groups: Vec::new(),
        }
    }

    /// Returns a reference to the list of groups.
    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    /// Parses the data and extracts the groups and tags.
    ///
    /// The groups and tags are stored in the `groups` field.
    ///
    /// # Example
    ///
    /// ```
    /// use tag_parser::TagParser;
    /// let data = include_str!("test_data.txt");
    /// let mut parser = TagParser::from(data);
    /// parser.parse();
    /// let groups = parser.groups();
    /// ```
    pub fn parse(&mut self) {
        let mut group = Group {
            name: String::new(),
            tags: Vec::new(),
        };

        for line in self
            .data
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            if line.starts_with('[') {
                if !group.name.is_empty() {
                    self.groups.push(group);
                    group = Group {
                        name: String::new(),
                        tags: Vec::new(),
                    };
                }

                let line = line.split('#').next().unwrap().trim();
                group.name = line[1..line.len() - 1].trim().to_string();
            } else if !line.starts_with('[') && group.name.is_empty() {
                continue; // Skip orphan tags
            } else {
                group
                    .tags
                    .push(line.split('#').next().unwrap().trim().to_string());
            }
        }

        self.groups.push(group);
    }
}

impl From<&str> for TagParser {
    /// Creates a new `TagParser` instance from a string.
    ///
    /// # Arguments
    ///
    /// * `data`: A string containing the tags.
    ///
    /// returns: A new `TagParser` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use tag_parser::TagParser;
    /// let data = include_str!("test_data.txt");
    /// let parser = TagParser::from(data);
    /// let groups = parser.groups();
    /// ```
    fn from(data: &str) -> Self {
        let mut parser = Self {
            data: data.to_string(),
            groups: Vec::new(),
        };

        parser.parse();
        parser
    }
}

impl From<String> for TagParser {
    /// Creates a new `TagParser` instance from a string.
    ///
    /// # Arguments
    ///
    /// * `data`: A string containing the tags.
    ///
    /// returns: A new `TagParser` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use tag_parser::TagParser;
    /// let data = include_str!("test_data.txt");
    /// let parser = TagParser::from(data);
    /// let groups = parser.groups();
    /// ```
    fn from(data: String) -> Self {
        Self::from(data.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_test() {
        let data = include_str!("test_data.txt");
        let parser = TagParser::from(data);
        let groups = parser.groups();
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].name, "Generic");
        assert_eq!(groups[0].tags.len(), 3);
        assert_eq!(groups[1].name, "IDs");
        assert_eq!(groups[1].tags.len(), 1);
        assert_eq!(groups[2].name, "EmptyGroup");
        assert_eq!(groups[2].tags.len(), 0);
    }

    #[test]
    fn multiple_groups_test() {
        let data = include_str!("test_data.txt");
        let parser = TagParser::from(data);
        let groups = parser.groups();
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].name, "Generic");
        assert_eq!(groups[0].tags.len(), 3);
        assert_eq!(groups[1].name, "IDs");
        assert_eq!(groups[1].tags.len(), 1);
    }

    #[test]
    fn tag_test() {
        let data = include_str!("test_data.txt");
        let parser = TagParser::from(data);
        let groups = parser.groups();
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].name, "Generic");
        assert_eq!(groups[0].tags.len(), 3);

        let tags = &groups[0].tags;
        assert_eq!(tags[0].as_str(), "red_hair female dress");
        assert_eq!(tags[1].as_str(), "dancing fire smile");
        assert_eq!(tags[2].as_str(), "進撃の巨人");

        let tags = &groups[1].tags;
        assert_eq!(tags[0].as_str(), "102349");
    }
}
