#[cfg(feature = "BiSolidCalculator")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCalculator")]
/// *This icon requires the feature* `BiSolidCalculator` *to be enabled*.
#[component]
pub fn Calculator(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 22h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2zm3-3H7v-2h2v2zm0-4H7v-2h2v2zm0-4H7V9h2v2zm4 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V9h2v2zm4 8h-2v-6h2v6zm0-8h-2V9h2v2zM6 4h12v3H6V4z" /></svg>
   }
}