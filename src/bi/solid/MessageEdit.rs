#[cfg(feature = "BiSolidMessageEdit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageEdit")]
/// *This icon requires the feature* `BiSolidMessageEdit` *to be enabled*.
#[component]
pub fn MessageEdit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .894-2 1.992v12.016C2 17.106 2.897 18 4 18h3v4l6.351-4H20c1.103 0 2-.894 2-1.992V3.992A1.998 1.998 0 0 0 20 2zM8.999 14.999H7V13l5.53-5.522 1.998 1.999-5.529 5.522zm6.472-6.464-1.999-1.999 1.524-1.523 1.999 1.999-1.524 1.523z" /></svg>
   }
}