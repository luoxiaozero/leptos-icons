#[cfg(feature = "IoEgg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEgg")]
/// *This icon requires the feature* `IoEgg` *to be enabled*.
#[component]
pub fn Egg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,480c-52.57,0-96.72-17.54-127.7-50.73C96.7,395.4,80,346.05,80,286.55,80,230.5,101.48,168,138.93,115,175.65,63,219.41,32,256,32s80.35,31,117.07,83C410.52,168,432,230.5,432,286.55c0,59.5-16.7,108.85-48.3,142.72C352.72,462.46,308.57,480,256,480Z" /></svg>
   }
}