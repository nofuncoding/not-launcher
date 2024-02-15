use druid::{
    commands, AppDelegate, Command, DelegateCtx, Env, Handled, Target, WindowId
};

use crate::{
    data::{AppState, Config},
    ui,
};

pub struct Delegate {
    main_window: Option<WindowId>,
    preferences_window: Option<WindowId>,
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env
    ) -> Handled {
        todo!()
    }
}