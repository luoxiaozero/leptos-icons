#[cfg(feature = "TbMoodSing")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodSing")]
/// *This icon requires the feature* `TbMoodSing` *to be enabled*.
#[component]
pub fn MoodSing(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-sing" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M9 9h.01" /><path d="M15 9h.01" /><path d="M15 15m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /></svg>
   }
}