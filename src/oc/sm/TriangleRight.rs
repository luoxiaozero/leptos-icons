#[cfg(feature = "OcSmTriangleRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTriangleRight")]
/// *This icon requires the feature* `OcSmTriangleRight` *to be enabled*.
#[component]
pub fn TriangleRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="m6.427 4.427 3.396 3.396a.25.25 0 0 1 0 .354l-3.396 3.396A.25.25 0 0 1 6 11.396V4.604a.25.25 0 0 1 .427-.177Z" /></svg>
   }
}