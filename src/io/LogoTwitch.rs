#[cfg(feature = "IoLogoTwitch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoTwitch")]
/// *This icon requires the feature* `IoLogoTwitch` *to be enabled*.
#[component]
pub fn LogoTwitch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M80,32,48,112V416h96v64h64l64-64h80L464,304V32ZM416,288l-64,64H256l-64,64V352H112V80H416Z" /><rect x="320" y="143" width="48" height="129" /><rect x="208" y="143" width="48" height="129" /></svg>
   }
}