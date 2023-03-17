#[cfg(feature = "RiOthersFillDoor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiOthersFillDoor")]
/// *This icon requires the feature* `RiOthersFillDoor` *to be enabled*.
#[component]
pub fn Door(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M18 3c.552 0 1 .448 1 1v16c0 .552-.448 1-1 1H6c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h12zm-4 8c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z" /></g></svg>
   }
}