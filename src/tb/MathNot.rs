#[cfg(feature = "TbMathNot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMathNot")]
/// *This icon requires the feature* `TbMathNot` *to be enabled*.
#[component]
pub fn MathNot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-math-not" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 12h14v4" /></svg>
   }
}