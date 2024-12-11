use crate::database::common::error::{BackendError, DbError, DbResultSingle, EntityError};
use crate::database::common::{EntityById};
use crate::database::common::query_parameters::DbQueryParams;

pub fn generate_query_param_string(params: &DbQueryParams) -> String {
    let mut qp_string = String::new();
    if !params.fetch_deleted {
        qp_string.push_str("AND bike.deleted_at IS NULL\n");
    }


    if let Some(order) = &params.order {
        qp_string.push_str("ORDER BY ");
        if let Some(table) = &order.table {
            qp_string.push_str(table.to_string().as_str());
            qp_string.push('.');
        }
        qp_string.push_str(order.column.to_string().as_str());
        qp_string.push(' ');
        qp_string.push_str(order.order.to_string().as_str());
    }
    qp_string.push('\n');
    if let Some(l) = params.limit {
        qp_string.push_str("LIMIT ");
        qp_string.push_str(l.to_string().as_str());
    }
    qp_string.push('\n');
    if let Some(o) = params.offset {
        qp_string.push_str("OFFSET ");
        qp_string.push_str(o.to_string().as_str());
    }
    qp_string
}

pub fn entity_is_correct<T: EntityById>(
    entity: Option<T>,
    error: EntityError,
    fetch_deleted: bool,
) -> DbResultSingle<T> {
    if let Some(ent) = entity {
        if fetch_deleted || !ent.is_deleted() {
            return Ok(ent);
        }
        return Err(DbError::from(BackendError::new(error.deleted)));
    }

    Err(DbError::from(BackendError::new(error.does_not_exist)))
}
