/// Adapted from https://github.com/jotare/bevy_quit
use bevy::app::AppExit;
use bevy::prelude::*;

/// Support multiple key bindings to exit a bevy game
pub struct QuitPlugin {
    bindings: QuitKeyBindings,
}

#[derive(Clone, Resource)]
struct QuitKeyBindings(Vec<KeyBinding>);

impl QuitPlugin {
    fn new() -> Self {
        Self {
            bindings: QuitKeyBindings(Vec::new()),
        }
    }

    /// Add a key binding to quit a bevy game
    pub fn add_key_binding<K: Into<KeyBinding>>(mut self, keybinding: K) -> Self {
        self.bindings.0.push(keybinding.into());
        self
    }
}

impl Default for QuitPlugin {
    /// Set C-q (ControlLeft, Q) as default quit key binding
    fn default() -> Self {
        Self::new().add_key_binding((KeyCode::ControlLeft, KeyCode::KeyQ))
    }
}

impl Plugin for QuitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<QuitKeyBindings>(self.bindings.clone())
            .add_systems(Update, quit_plugin);
    }
}

fn quit_plugin(
    input: Res<ButtonInput<KeyCode>>,
    quit_bindings: Res<QuitKeyBindings>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for binding in quit_bindings.0.iter() {
        let should_exit = match binding {
            KeyBinding::Single(key) => input.pressed(*key),
            KeyBinding::Multi(keys) => keys.iter().all(|key| input.pressed(*key)),
        };
        if should_exit {
            app_exit_events.send(AppExit::Success);
            return;
        }
    }
}

/// Representation of a key binding (from 1 to N simultaneous keys pressed at
/// the same time)
#[derive(Clone)]
pub enum KeyBinding {
    Single(KeyCode),
    Multi(Vec<KeyCode>),
}

/// Single key binding (e.g. ESC)
impl From<KeyCode> for KeyBinding {
    fn from(value: KeyCode) -> Self {
        Self::Single(value)
    }
}

/// Common modifier+character keybindings (e.g. C-q)
impl From<(KeyCode, KeyCode)> for KeyBinding {
    fn from(value: (KeyCode, KeyCode)) -> Self {
        Self::Multi(vec![value.0, value.1])
    }
}

/// Common 2 modifiers and 1 character (e.g. C-M-q)
impl From<(KeyCode, KeyCode, KeyCode)> for KeyBinding {
    fn from(value: (KeyCode, KeyCode, KeyCode)) -> Self {
        Self::Multi(vec![value.0, value.1, value.2])
    }
}

/// Unlimited number of simultaneous keys
impl From<Vec<KeyCode>> for KeyBinding {
    fn from(value: Vec<KeyCode>) -> Self {
        Self::Multi(value)
    }
}
