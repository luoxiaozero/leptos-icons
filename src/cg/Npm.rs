#[cfg(feature = "CgNpm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgNpm")]
/// *This icon requires the feature* `CgNpm` *to be enabled*.
#[component]
pub fn Npm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 21C3.89543 21 3 20.1046 3 19V5C3 3.89543 3.89543 3 5 3H19C20.1046 3 21 3.89543 21 5V19C21 20.1046 20.1046 21 19 21H5ZM6 18V6H18V18H15V9H12V18H6Z" fill="currentColor" /></svg>
   }
}