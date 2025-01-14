use leptos::*;

#[component]
pub fn LayoutHeader(
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="thaw-layout-header" style=move || style.get()>
            {children()}
        </div>
    }
}
