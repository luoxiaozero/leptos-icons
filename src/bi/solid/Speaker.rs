#[cfg(feature = "BiSolidSpeaker")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSpeaker")]
/// *This icon requires the feature* `BiSolidSpeaker` *to be enabled*.
#[component]
pub fn Speaker(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="15" r="3" /><path d="M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm-6 2a2 2 0 1 1-2 2 2 2 0 0 1 2-2zm0 16a5 5 0 1 1 5-5 5 5 0 0 1-5 5z" /></svg>
   }
}