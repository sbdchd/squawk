use crate::global_state::GlobalState;

pub(crate) fn handle_shutdown(state: &mut GlobalState, _params: ()) {
    state.request_shutdown();
}
