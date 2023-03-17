#[cfg(feature = "SiAlgorand")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAlgorand")]
/// *This icon requires the feature* `SiAlgorand` *to be enabled*.
#[component]
pub fn Algorand(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M13.874 0h3.673l1.61 5.963h3.789l-2.588 4.5 3.624 13.533h-3.757l-2.44-9.077-5.247 9.079H8.345l8.107-14.051-1.304-4.878L4.215 24H.018Z" /></svg>
   }
}