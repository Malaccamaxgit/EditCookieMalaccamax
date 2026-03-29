//! Options page module

use leptos::prelude::*;

pub mod preferences;

pub use preferences::PreferencesForm;

#[component]
pub fn App() -> impl IntoView {
    use crate::core::storage;
    use crate::shared::helpers;
    use crate::shared::types::Preferences;

    helpers::ensure_stylesheets("css/options.css");

    let (preferences, set_preferences) = signal(Preferences::default());

    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let prefs = storage::PreferencesStorage::get().await;
            set_preferences.set(prefs.clone());
            helpers::apply_theme(&prefs.theme);
        });
    });

    Effect::new(move |_| {
        let prefs = preferences.get();
        helpers::apply_theme(&prefs.theme);
    });

    view! {
        <div class="options-container">
            <h1>"Edit Cookies - Settings"</h1>
            <PreferencesForm preferences=preferences set_preferences=set_preferences />
        </div>
    }
}
