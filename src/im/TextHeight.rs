#[cfg(feature = "ImTextHeight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTextHeight")]
/// *This icon requires the feature* `ImTextHeight` *to be enabled*.
#[component]
pub fn TextHeight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 12h2l-2.5 3-2.5-3h2v-8h-2l2.5-3 2.5 3h-2zM10 1v4l-1-2h-3v11h2v1h-6v-1h2v-11h-3l-1 2v-4z" /></svg>
   }
}