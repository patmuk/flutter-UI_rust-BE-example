use crate::application::api::lifecycle::Lifecycle;
pub use crate::application::processing_errors::ProcessingError;
pub use crate::domain::effects::Effect;
use crate::utils::cqrs_traits::Cqrs;
use log::debug;

pub fn process_cqrs<C: Cqrs>(cqrs: C) -> Result<Vec<Effect>, ProcessingError> {
    let is_command = cqrs.is_command();
    if is_command {
        debug!("Processing cqrs_command: {:?}", cqrs);
    } else {
        debug!("Processing cqrs_query: {:?}", cqrs);
    }
    let lifecycle = Lifecycle::get();
    let effects = cqrs.process(&lifecycle.app_state);
    debug!(
        "Processed cqrs, new model {:?}",
        lifecycle.app_state.model.blocking_read()
    );
    debug!("got the effects {:?}", effects);
    if is_command {
        lifecycle.app_state.mark_dirty();
        // persist change to not miss it
        lifecycle
            .app_state
            .persist(&lifecycle.app_config.app_state_file_path)
            .unwrap(); // TODO convert to own error
                       // Ok::<_, dyn ProcessingError>(effects)
    }
    effects
}
