#[cfg(feature = "BiSolidTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTag")]
/// *This icon requires the feature* `BiSolidTag` *to be enabled*.
#[component]
pub fn Tag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21.842 6.218a1.977 1.977 0 0 0-.424-.628A1.99 1.99 0 0 0 20 5H8c-.297 0-.578.132-.769.359l-5 6c-.309.371-.309.91 0 1.281l5 6c.191.228.472.36.769.36h12a1.977 1.977 0 0 0 1.41-.582A1.99 1.99 0 0 0 22 17V7c0-.266-.052-.525-.158-.782z" /></svg>
   }
}