#[cfg(feature = "TbCircuitBulb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircuitBulb")]
/// *This icon requires the feature* `TbCircuitBulb` *to be enabled*.
#[component]
pub fn CircuitBulb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circuit-bulb" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M2 12h5" /><path d="M17 12h5" /><path d="M12 12m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M8.5 8.5l7 7" /><path d="M15.5 8.5l-7 7" /></svg>
   }
}