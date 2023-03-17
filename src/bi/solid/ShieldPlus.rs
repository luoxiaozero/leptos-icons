#[cfg(feature = "BiSolidShieldPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidShieldPlus")]
/// *This icon requires the feature* `BiSolidShieldPlus` *to be enabled*.
#[component]
pub fn ShieldPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m20.43 5.76-8-3.56a1 1 0 0 0-.82 0l-8 3.56a1 1 0 0 0-.59.9c0 2.37.44 10.8 8.51 15.11a1 1 0 0 0 1 0c8-4.3 8.58-12.71 8.57-15.1a1 1 0 0 0-.67-.91zm-4.43 7h-3v3h-2v-3H8v-2h3v-3h2v3h3z" /></svg>
   }
}