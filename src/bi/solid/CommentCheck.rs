#[cfg(feature = "BiSolidCommentCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCommentCheck")]
/// *This icon requires the feature* `BiSolidCommentCheck` *to be enabled*.
#[component]
pub fn CommentCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-9 11.914-3.707-3.707 1.414-1.414L11 11.086l4.793-4.793 1.414 1.414L11 13.914z" /></svg>
   }
}