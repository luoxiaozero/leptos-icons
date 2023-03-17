#[cfg(feature = "HiMdSolidClipboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidClipboard")]
/// *This icon requires the feature* `HiMdSolidClipboard` *to be enabled*.
#[component]
pub fn Clipboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.8871 3.18189C14.283 3.21913 14.6773 3.262 15.07 3.31043C16.1942 3.44911 17 4.41371 17 5.51659V16.75C17 17.9926 15.9926 19 14.75 19H5.25C4.00736 19 3 17.9926 3 16.75V5.51659C3 4.41371 3.80579 3.44911 4.93005 3.31043C5.32266 3.26201 5.71697 3.21913 6.11291 3.18189C6.46903 1.92267 7.62676 1 9 1H11C12.3732 1 13.531 1.92267 13.8871 3.18189ZM7.5 4C7.5 3.17157 8.17157 2.5 9 2.5H11C11.8284 2.5 12.5 3.17157 12.5 4V4.5H7.5V4Z" fill="#0F172A" /></svg>
   }
}