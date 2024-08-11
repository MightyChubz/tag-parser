use std::path::Path;

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct TagParser {
    data: String,
    groups: Vec<Group>,
}

impl TagParser {
    pub fn new(path: &Path) -> Self {
        let data = std::fs::read_to_string(path).unwrap();
        Self {
            data,
            groups: Vec::new(),
        }
    }

    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

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
