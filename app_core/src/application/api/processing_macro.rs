use std::io;

use super::lifecycle::{Effect, Lifecycle};
use crate::{
    application::app_state::AppState,
    domain::todo_list::{TodoListModel, TodoListProcessingError},
};
use generate_api_macros::generate_api;

generate_api!("app_core/src/domain/effects.rs");
