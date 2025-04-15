use super::server::get_login;
use crate::app::models::auth::{Role, User};
use dioxus::prelude::*;
use std::collections::HashSet;

pub fn use_user_provider() {
    let login_res = use_resource(get_login);
    let login = use_memo(move || {
        login_res()
            .map(|u| u.ok().unwrap_or_default())
            .unwrap_or_default()
    });

    use_context_provider(|| login);
}

pub fn use_user() -> Option<User> {
    let context = use_context::<Memo<Option<User>>>();
    context()
}

pub fn use_role() -> Option<Role> {
    use_user().map(|u| u.role)
}

pub fn use_permissions(check_role: Role) -> bool {
    let role = use_role();

    match role {
        Some(role) => HashSet::from(role).contains(&check_role),
        None => false,
    }
}

pub fn use_loggedin() -> bool {
    let user = use_user();
    user.is_some()
}
