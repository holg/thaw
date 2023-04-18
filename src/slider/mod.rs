use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Slider(
    cx: Scope,
    #[prop(optional, into)] value: MaybeSignal<f64>,
    #[prop(optional)] on_value: Option<SignalSetter<String>>,
    #[prop(default = MaybeSignal::Static(100f64), into)] max: MaybeSignal<f64>,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = theme.common.color_primary;
        css_vars.push_str(&format!("--background-color-fill: {bg_color};"));

        css_vars
    });

    let percentage = create_memo(cx, move |_| {
        if value.get() < 0.0 || max.get() <= 0.0 {
            0f64
        } else if value.get() >= max.get() {
            100f64
        } else {
            value.get() / max.get() * 100.0
        }
    });
    let class_name = mount_style("slider", || style_sheet_str!("./src/slider/slider.css"));

    view! {cx, class=class_name,
        <div class="melt-slider" style=move || css_vars.get()>
            <div class="melt-slider-rail">
                <div class="melt-slider-rail__fill" style=move || format!("width: {}%", percentage.get())></div>
            </div>
            <div class="melt-slider-handle" style=move || format!("left: {}%; transform: translateX(-{}%)", percentage.get(), percentage.get())>
            </div>
        </div>
    }
}