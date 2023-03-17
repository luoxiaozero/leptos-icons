#[cfg(feature = "BiSolidBatteryLow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBatteryLow")]
/// *This icon requires the feature* `BiSolidBatteryLow` *to be enabled*.
#[component]
pub fn BatteryLow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 8a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2h2v-4h-2V8zM5 15V9h3l4 6H5z" /></svg>
   }
}