#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_test() {
        let data = include_str!("test_data.txt");
        let parser = TagParser::from(data);
        let groups = parser.groups();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "Generic");
        assert_eq!(groups[0].tags.len(), 3);
    }

    #[test]
    fn multiple_groups_test() {
        let data = include_str!("test_data.txt");
        let parser = TagParser::from(data);
        let groups = parser.groups();
        assert_eq!(groups.len(), 2);
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
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "Generic");
        assert_eq!(groups[0].tags.len(), 3);

        let tags = &groups[0].tags;
        assert_eq!(tags[0].name, "red_hair female dress");
        assert_eq!(tags[1].name, "dancing fire smile");
        assert_eq!(tags[2].name, "進撃の巨人");

        let tags = &groups[1].tags;
        assert_eq!(tags[0].name, "102349");
    }
}
