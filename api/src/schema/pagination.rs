use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// `?page=1&per_page=10&search=foo` on any list endpoint
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

// ponytail: high ceiling so the attendance roll can still load every student in one call,
// lower it once the roll gets student names from the API instead
const MAX_PER_PAGE: i64 = 1000;
const DEFAULT_PER_PAGE: i64 = 10;

impl PageQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .clamp(1, MAX_PER_PAGE)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }

    /// `%term%` for ILIKE with the wildcards in the term escaped, None when there is no search
    pub fn pattern(&self) -> Option<String> {
        let term = self.search.as_deref()?.trim();
        if term.is_empty() {
            return None;
        }
        let escaped = term
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_");
        Some(format!("%{escaped}%"))
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: i64, query: &PageQuery) -> Self {
        Page {
            items,
            total,
            page: query.page(),
            per_page: query.per_page(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_and_escapes() {
        let query = PageQuery {
            page: Some(0),
            per_page: Some(5000),
            search: Some("  50%_off  ".into()),
        };
        assert_eq!(query.page(), 1);
        assert_eq!(query.per_page(), MAX_PER_PAGE);
        assert_eq!(query.offset(), 0);
        assert_eq!(query.pattern().as_deref(), Some("%50\\%\\_off%"));
        assert_eq!(PageQuery::default().pattern(), None);
        assert_eq!(PageQuery::default().per_page(), DEFAULT_PER_PAGE);
    }
}
