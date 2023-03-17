#[cfg(feature = "RiSystemFillStar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillStar")]
/// *This icon requires the feature* `RiSystemFillStar` *to be enabled*.
#[component]
pub fn Star(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 18.26l-7.053 3.948 1.575-7.928L.587 8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935 5.488 1.575 7.928z" /></g></svg>
   }
}