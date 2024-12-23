use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct DbQueryParams {
    pub order: Option<DbOrderColumn>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub ignore_hidden: Option<DbTable>,
}

impl DbQueryParams {
    #[allow(dead_code)]
    pub fn new(
        order: Option<DbOrderColumn>,
        limit: Option<i64>,
        offset: Option<i64>,
        ignore_hidden: Option<DbTable>,
    ) -> Self {
        Self {
            order,
            limit,
            offset,
            ignore_hidden,
        }
    }

    #[allow(dead_code)]
    pub fn limit(limit: i64, offset: i64, ignore_hidden: Option<DbTable>) -> Self {
        Self {
            order: Some(DbOrderColumn::default()),
            limit: Some(limit),
            offset: Some(offset),
            ignore_hidden,
        }
    }

    pub fn order(order: DbOrderColumn, ignore_hidden: Option<DbTable>) -> Self {
        Self {
            order: Some(order),
            limit: None,
            offset: None,
            ignore_hidden,
        }
    }
    #[allow(dead_code)]
    pub fn deleted() -> Self {
        Self {
            order: Some(DbOrderColumn::default()),
            limit: None,
            offset: None,
            ignore_hidden: None,
        }
    }
}

impl Default for DbQueryParams {
    fn default() -> Self {
        Self {
            order: Some(DbOrderColumn::default()),
            limit: None,
            offset: None,
            ignore_hidden: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbOrderColumn {
    pub table: Option<DbTable>,
    pub column: DbColumn,
    pub order: DbOrder,
}

#[allow(dead_code)]
impl DbOrderColumn {
    pub fn new(table: DbTable, column: DbColumn, order: DbOrder) -> Self {
        Self {
            table: Some(table),
            column,
            order,
        }
    }

    pub fn new_column_only(column: DbColumn, order: DbOrder) -> Self {
        Self {
            table: None,
            column,
            order,
        }
    }
}

impl Default for DbOrderColumn {
    fn default() -> Self {
        Self {
            table: None,
            column: DbColumn::Id,
            order: DbOrder::Desc,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DbTable {
    Bike,
    Model,
    Brand,
    BikeImage,
    User,
}

impl DbTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbTable::Bike => write!(f, "bike"),
            DbTable::BikeImage => write!(f, "image"),
            DbTable::Brand => write!(f, "brand"),
            DbTable::Model => write!(f, "model"),
            DbTable::User => write!(f, "user"),
        }
    }
}

impl Display for DbTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DbColumn {
    Id,
    Name,
    Length,
    ViewCount,
    LikeCount,
    CreatedAt,
    EditedAt,
    Ordering,
    Path,
    BrandName,
}

impl DbColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbColumn::Id => write!(f, "id"),
            DbColumn::Name => write!(f, "name"),
            DbColumn::Length => write!(f, "length"),
            DbColumn::ViewCount => write!(f, "view_count"),
            DbColumn::LikeCount => write!(f, "like_count"),
            DbColumn::CreatedAt => write!(f, "created_at"),
            DbColumn::EditedAt => write!(f, "edited_at"),
            DbColumn::Ordering => write!(f, "ordering"),
            DbColumn::Path => write!(f, "path"),
            DbColumn::BrandName => write!(f, "brand_name"),
        }
    }
}

impl Display for DbColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DbOrder {
    Asc,
    Desc,
}

impl DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbOrder::Asc => write!(f, "ASC"),
            DbOrder::Desc => write!(f, "DESC"),
        }
    }
}

impl Display for DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}
