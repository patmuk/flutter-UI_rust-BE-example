pub(crate) fn file_location_2_base_path(file_path: &str) -> String {
    let mut path_split = file_path.split('/').skip_while(|element| *element != "src");
    path_split
        .next()
        .expect("file path needs to contain 'src/'");
    let dirty_result = format!("crate::{}", path_split.collect::<Vec<&str>>().join("::"));
    match &dirty_result[dirty_result.rfind("::").unwrap()..] {
        "::mod.rs" => dirty_result[..dirty_result.len() - 8].to_string(),
        file if file.ends_with(".rs") => dirty_result[..dirty_result.len() - 3].to_string(),
        file => {
            panic!(
                "File location doesn't end with a '.rs' file: '{}'",
                &file[2..]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::file_location_2_base_path;

    #[test]
    #[should_panic(expected = "file path needs to contain 'src/'")]
    fn test_file_location_2_base_path_no_src() {
        let input = String::from("main.rs");
        file_location_2_base_path(&input);
    }

    #[test]
    #[should_panic(expected = "File location doesn't end with a '.rs' file: 'main'")]
    fn test_file_location_2_base_path_main_no_rs() {
        let input = String::from("src/module/main");
        file_location_2_base_path(&input);
    }
    #[test]
    #[should_panic(expected = "File location doesn't end with a '.rs' file: 'mod'")]
    fn test_file_location_2_base_path_no_rs() {
        let input = String::from("src/module/mod");
        file_location_2_base_path(&input);
    }
    #[test]
    fn test_file_location_2_base_path_zero_levels() {
        let input = String::from("src/main.rs");
        assert_eq!("crate::main".to_string(), file_location_2_base_path(&input));
    }
    #[test]
    fn test_file_location_2_base_path_one_level() {
        let input = String::from("src/domain/model.rs");
        assert_eq!(
            "crate::domain::model".to_string(),
            file_location_2_base_path(&input)
        );
    }
    #[test]
    fn test_file_location_2_base_path_two_levels() {
        let input = String::from("src/domain/model/item.rs");
        assert_eq!(
            "crate::domain::model::item".to_string(),
            file_location_2_base_path(&input)
        );
    }
    #[test]
    fn test_file_location_2_base_path_multiple_levels() {
        let input = String::from("src/domain/model/items/entity.rs");
        assert_eq!(
            "crate::domain::model::items::entity".to_string(),
            file_location_2_base_path(&input)
        );
    }
    #[test]
    fn test_file_location_2_base_path_one_level_with_mod() {
        let input = String::from("src/domain/model/mod.rs");
        assert_eq!(
            "crate::domain::model".to_string(),
            file_location_2_base_path(&input)
        );
    }
    #[test]
    fn test_file_location_2_base_path_two_levels_with_mod() {
        let input = String::from("src/domain/model/item/mod.rs");
        assert_eq!(
            "crate::domain::model::item".to_string(),
            file_location_2_base_path(&input)
        );
    }
    #[test]
    fn test_file_location_2_base_path_multiple_levels_with_mod() {
        let input = String::from("src/domain/model/items/entity/mod.rs");
        assert_eq!(
            "crate::domain::model::items::entity".to_string(),
            file_location_2_base_path(&input)
        );
    }
}
