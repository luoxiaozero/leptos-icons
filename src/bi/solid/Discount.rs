#[cfg(feature = "BiSolidDiscount")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDiscount")]
/// *This icon requires the feature* `BiSolidDiscount` *to be enabled*.
#[component]
pub fn Discount(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 5H3a1 1 0 0 0-1 1v4h.893c.996 0 1.92.681 2.08 1.664A2.001 2.001 0 0 1 3 14H2v4a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-4h-1a2.001 2.001 0 0 1-1.973-2.336c.16-.983 1.084-1.664 2.08-1.664H22V6a1 1 0 0 0-1-1zM9 9a1 1 0 1 1 0 2 1 1 0 1 1 0-2zm-.8 6.4 6-8 1.6 1.2-6 8-1.6-1.2zM15 15a1 1 0 1 1 0-2 1 1 0 1 1 0 2z" /></svg>
   }
}