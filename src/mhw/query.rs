//use super::common::*;
use itertools::Itertools;
use reqwest;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::fmt;

//
// Search Category
//
#[derive(Debug)]
pub enum SearchCategory {
    Armor,
    Weapon,
}

impl Default for SearchCategory {
    fn default() -> Self {
        SearchCategory::Armor
    }
}

impl fmt::Display for SearchCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchCategory::Armor => write!(f, "armor"),
            SearchCategory::Weapon => write!(f, "weapon"),
        }
    }
}

//
// Query Filter Type
//
#[derive(Debug)]
pub enum QueryFilterType {
    Exact(String),
    Like(String),
}

impl fmt::Display for QueryFilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryFilterType::Exact(val) => write!(f, "{}", val),
            QueryFilterType::Like(val) => write!(f, "{{\"$like\":\"{}\"}}", val),
        }
    }
}

//
// Query Filter
//
#[derive(Debug)]
pub struct QueryFilter {
    field_name: String,
    filter: QueryFilterType,
}

impl fmt::Display for QueryFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"{name}\":{filter}}}",
            name = self.field_name,
            filter = self.filter
        )
    }
}

//
// Query Prediction Meta
//
#[derive(Debug)]
pub enum QueryProjectionMeta {
    Inclusive,
    Exclusive,
}

impl Default for QueryProjectionMeta {
    fn default() -> Self {
        QueryProjectionMeta::Inclusive
    }
}

//
// Query Prediction
//
#[derive(Debug, Default)]
pub struct QueryProjection {
    meta: QueryProjectionMeta,
    fields: Vec<&'static str>,
}

impl QueryProjection {
    fn new() -> Self {
        Self {
            meta: QueryProjectionMeta::Inclusive,
            fields: vec![],
        }
    }
}

impl fmt::Display for QueryProjection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bool_str = match self.meta {
            QueryProjectionMeta::Inclusive => "true",
            QueryProjectionMeta::Exclusive => "false",
        };

        write!(
            f,
            "{{{}}}",
            self.fields
                .iter()
                .format_with(",", |data, f| f(&format_args!("\"{}\":{}", data, bool_str)))
        )
    }
}

//
// Query Info
//
#[derive(Debug)]
pub struct QueryInfo {
    category: SearchCategory,
    filter: QueryFilter,
    projection: Option<QueryProjection>,
}

impl QueryInfo {
    pub fn find_ids(text: &str) -> Self {
        Self {
            category: Default::default(),
            filter: QueryFilter {
                field_name: "name".to_owned(),
                filter: QueryFilterType::Like(text.to_owned()),
            },
            projection: Some(QueryProjection {
                meta: QueryProjectionMeta::Inclusive,
                fields: vec!["id", "name", "type"],
            }),
        }
    }
}

//
// MHW Query Error
//
pub enum MHWQueryError {
    Internal(String),
    API(String),
    Unimplemented,
}

impl fmt::Display for MHWQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MHWQueryError::Unimplemented => write!(f, "Unimplemented!"),
            MHWQueryError::Internal(s) => write!(f, "Internal error: {}", s),
            MHWQueryError::API(s) => write!(f, "API error: {}", s),
        }
    }
}

//
// Execute Query...
//
// TODO: Should this be a function on the struct instead?
pub fn execute_mhw_query<T>(info: QueryInfo) -> Result<T, MHWQueryError>
where
    T: DeserializeOwned,
{
    let mut url_string = format!(
        "https://mhw-db.com/{category}?q=",
        category = info.category.to_string()
    );

    let filter_string = info.filter.to_string();
    url_string.push_str(filter_string.as_str());

    if let Some(proj) = info.projection {
        let projection_string = format!("&p={}", proj);
        url_string.push_str(projection_string.as_str());
    }

    // let filter_string = format!(
    //     "{{\"{name}\"}}:{value}",
    //     name = filter.0,
    //     value = filter.1.filter_string()
    // );

    // let query_string = format!(
    //     "{{\"name\":{{\"$like\":\"{text}\"}}}}&p={{\"id\":true,\"name\":true,\"type\":true}}",
    //     text = urlencoding::encode(&info.like_text.unwrap().to_owned()).as_str()
    // );

    let url = Url::parse(url_string.as_str()).unwrap();
    println!("{}", url.as_str());

    let mut result = match reqwest::get(url) {
        Ok(r) => r,
        Err(e) => return Err(MHWQueryError::API(format!("Error querying API: {}", e))),
    };

    // let json_result = result
    //     .json()
    //     .map_err(|_json_err| MHWQueryError::Internal("JSON error".to_string()));
    match result.json() {
        Ok(r) => Ok(r),
        Err(e) => Err(MHWQueryError::Internal(format!(
            "Error converting API search into Vec: {}",
            e
        ))),
    }
}
