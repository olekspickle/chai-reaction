#[macro_export]
macro_rules! return_if_at_limit {
    ($iterable:expr, $max_count:expr) => {
        if $iterable.into_iter().count() >= $max_count {
            warn!(format!("{:?} reached max count {}", $iterable, $max_count),);
            return;
        }
    };
}

#[macro_export]
macro_rules! single_else_return {
    ($query:expr) => {
        match $query.single() {
            Ok(item) => item,
            Err(error) => {
                error!("error getting single {:?}: {}", $query, error);
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! single_mut_else_return {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(item) => item,
            Err(error) => {
                error!("error getting single mut {:?}: {}", $query, error);
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! get_entity_else_return {
    ($query:expr, $entity:expr, $item_type:ty) => {{
        use std::any::type_name;
        let type_name = type_name::<$item_type>();
        match $query.get($entity) {
            Ok(item) => item,
            Err(_) => {
                error!(EntityError::EntityNotInQuery(format!(
                    "couldn't fetch entity of type {} from query",
                    type_name
                )));
                return;
            }
        }
    }};
}

#[macro_export]
macro_rules! get_mut_entity_else_return {
    ($query:expr, $entity:expr, $item_type:ty) => {{
        use std::any::type_name;
        let type_name = type_name::<$item_type>();
        match $query.get_mut($entity) {
            Ok(item) => item,
            Err(_) => {
                error!(EntityError::EntityNotInQuery(&format!(
                    "couldn't fetch entity of type {} from query (mut)",
                    type_name
                )));
                return;
            }
        }
    }};
}
