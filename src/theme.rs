use leptos::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Theme::System => "◐",
            Theme::Light => "☀",
            Theme::Dark => "☾",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }
}

const STORAGE_KEY: &str = "theme";

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

fn document_element() -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
}

pub fn load_theme() -> Theme {
    local_storage()
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .map(|v| Theme::from_str(&v))
        .unwrap_or(Theme::System)
}

pub fn apply_theme(theme: Theme) {
    if let Some(html) = document_element() {
        match theme {
            Theme::System => {
                let _ = html.remove_attribute("data-theme");
            }
            other => {
                let _ = html.set_attribute("data-theme", other.as_str());
            }
        }
    }
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(STORAGE_KEY, theme.as_str());
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = RwSignal::new(load_theme());
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        apply_theme(theme.get());
    });

    let options = [Theme::System, Theme::Light, Theme::Dark];

    view! {
        <div class="theme-toggle" aria-label="Theme">
            <button
                type="button"
                class="theme-button"
                aria-haspopup="listbox"
                aria-expanded=move || if open.get() { "true" } else { "false" }
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <span class="theme-icon">{move || theme.get().icon()}</span>
                <span class="theme-name">{move || theme.get().name()}</span>
                <span class="caret">"▾"</span>
            </button>
            {move || {
                if !open.get() {
                    ().into_any()
                } else {
                    view! {
                        <ul class="theme-menu" role="listbox">
                            {options.into_iter().map(|t| {
                                let selected = move || theme.get() == t;
                                view! {
                                    <li>
                                        <button
                                            type="button"
                                            class:selected=selected
                                            role="option"
                                            aria-selected=move || if selected() { "true" } else { "false" }
                                            on:click=move |_| {
                                                theme.set(t);
                                                open.set(false);
                                            }
                                        >
                                            <span class="theme-icon">{t.icon()}</span>
                                            <span class="theme-name">{t.name()}</span>
                                        </button>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }
                    .into_any()
                }
            }}
        </div>
    }
}
