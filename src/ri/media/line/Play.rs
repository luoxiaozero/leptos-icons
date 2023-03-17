#[cfg(feature = "RiMediaLinePlay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLinePlay")]
/// *This icon requires the feature* `RiMediaLinePlay` *to be enabled*.
#[component]
pub fn Play(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16.394 12L10 7.737v8.526L16.394 12zm2.982.416L8.777 19.482A.5.5 0 0 1 8 19.066V4.934a.5.5 0 0 1 .777-.416l10.599 7.066a.5.5 0 0 1 0 .832z" /></g></svg>
   }
}