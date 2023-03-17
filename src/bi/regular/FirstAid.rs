#[cfg(feature = "BiRegularFirstAid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFirstAid")]
/// *This icon requires the feature* `BiRegularFirstAid` *to be enabled*.
#[component]
pub fn FirstAid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 6h-3V4c0-1.103-.897-2-2-2H9c-1.103 0-2 .897-2 2v2H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V8c0-1.103-.897-2-2-2zM9 4h6v2H9V4zM4 18V8h16l.001 10H4z" /><path d="M13 9h-2v3H8v2h3v3h2v-3h3v-2h-3z" /></svg>
   }
}