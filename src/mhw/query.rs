use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use reqwest;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::fmt;
use urlencoding;

//
// Search Category
//
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum SearchCategory {
    Armor = 0,
    Weapons,
    MAX,
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
            SearchCategory::Weapons => write!(f, "weapons"),
            _ => write!(f, "ERROR"),
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
            QueryFilterType::Exact(val) => write!(f, "{}", urlencoding::encode(val)),
            QueryFilterType::Like(val) => {
                write!(f, "{{\"$like\":\"{}\"}}", urlencoding::encode(val))
            }
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

impl QueryFilter {
    pub fn new(field: String, filter: QueryFilterType) -> Self {
        Self {
            field_name: field,
            filter: filter,
        }
    }
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
#[allow(dead_code)]
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
    filter: Option<QueryFilter>,
    projection: Option<QueryProjection>,
}

impl QueryInfo {
    pub fn find_ids(text: &str, category: SearchCategory) -> Self {
        Self {
            category,
            filter: Some(QueryFilter {
                field_name: "name".to_owned(),
                filter: QueryFilterType::Like(text.to_owned()),
            }),
            projection: Some(QueryProjection {
                meta: QueryProjectionMeta::Inclusive,
                fields: vec!["id", "name", "type"],
            }),
        }
    }

    pub fn find_category(category: SearchCategory) -> Self {
        Self {
            category: category,
            filter: Default::default(),
            projection: Default::default(),
        }
    }

    pub fn with_filter(mut self, filter: QueryFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    #[allow(dead_code)]
    pub fn with_projection(mut self, proj: QueryProjection) -> Self {
        self.projection = Some(proj);
        self
    }

    pub fn execute_mhw_query<T>(&self) -> Result<T, MHWQueryError>
    where
        T: DeserializeOwned,
    {
        let mut url_string = format!(
            "https://mhw-db.com/{category}",
            category = self.category.to_string()
        );

        let mut prefix = "?"; // in case there's no query, prefix should use '?'
        if let Some(filter) = &self.filter {
            let filter_string = format!("?q={}", filter);
            url_string.push_str(filter_string.as_str());
            prefix = "&";
        }

        if let Some(proj) = &self.projection {
            let projection_string = format!("{}p={}", prefix, proj);
            url_string.push_str(projection_string.as_str());
        }

        let url = Url::parse(url_string.as_str()).unwrap();
        println!("{}", url.as_str());

        let mut result = match reqwest::get(url) {
            Ok(r) => r,
            Err(e) => return Err(MHWQueryError::API(format!("Error querying API: {}", e))),
        };

        match result.json() {
            Ok(r) => Ok(r),
            Err(e) => Err(MHWQueryError::Internal(format!(
                "Error converting API search into Vec: {}",
                e
            ))),
        }
    }
}

//
// MHW Query Error
//
pub enum MHWQueryError {
    Internal(String),
    API(String),
    #[allow(dead_code)]
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
