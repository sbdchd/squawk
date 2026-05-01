use anyhow::Result;

use crate::global_state::GlobalState;

pub(crate) fn handle_shutdown(state: &mut GlobalState, _params: ()) -> Result<()> {
    state.request_shutdown();

    Ok(())
}
