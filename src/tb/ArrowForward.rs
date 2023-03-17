#[cfg(feature = "TbArrowForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowForward")]
/// *This icon requires the feature* `TbArrowForward` *to be enabled*.
#[component]
pub fn ArrowForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-forward" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 11l4 4l-4 4m4 -4h-11a4 4 0 0 1 0 -8h1" /></svg>
   }
}