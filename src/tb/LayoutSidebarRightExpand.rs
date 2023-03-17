#[cfg(feature = "TbLayoutSidebarRightExpand")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLayoutSidebarRightExpand")]
/// *This icon requires the feature* `TbLayoutSidebarRightExpand` *to be enabled*.
#[component]
pub fn LayoutSidebarRightExpand(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-layout-sidebar-right-expand" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z" /><path d="M15 4v16" /><path d="M10 10l-2 2l2 2" /></svg>
   }
}