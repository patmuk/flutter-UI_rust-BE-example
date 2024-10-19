use super::todo_list::TodoListModel;
use crate::application::bridge::frb_generated::RustAutoOpaque;

/// All effects for the same reason all Processing Errors are in one emun:
/// - easier handling for the consumer (match)
/// - reuse of effects among domain implementations
// #[derive(Debug)]
pub enum Effect {
    // Parameters need to be owned by `Effect`.
    // The attributes live in the app state - we don't want to
    // send them back and furth.
    // A reference is hard to manage - we could only `& mut` it when all
    // `&` are released, which only happens via `.dispose()` on the dart side.
    //
    // Thus, the best approach is providing a shared reference, which
    // Dart can not directly read: `RustAutoOpaque`, which is more or less a `Arc<RwLock>`.
    // Dart can access the lightweight properties needed for presentation with a function call
    // on this reference.
    //
    // In strict CQRS a command should not return a value.
    // However, this safes a consecutive query.
    // Thus, return only data for which a query exists.
    RenderTodoList(RustAutoOpaque<TodoListModel>),
}
