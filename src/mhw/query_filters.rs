use super::search::SearchCategory;
use imgui::*;

pub fn get_filter_categories(search_category: &SearchCategory) -> Vec<ImString> {
    match search_category {
        _ => {
            let mut v = Vec::default();
            v.push("Filter A".to_owned().into());
            v
        }
    }
}
