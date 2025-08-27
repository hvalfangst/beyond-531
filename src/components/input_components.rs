use leptos::*;

#[component]
pub fn InputField(
    label: String,
    value: ReadSignal<f64>,
    on_change: WriteSignal<f64>,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 0.0)] min: f64,
) -> impl IntoView {
    view! {
        <div class="input-field">
            <label>{label}</label>
            <input
                type="number"
                step=step
                min=min
                value=move || value.get()
                on:input=move |ev| {
                    if let Ok(new_value) = event_target_value(&ev).parse::<f64>() {
                        leptos::logging::log!("Input changed to: {}", new_value);
                        on_change.set(new_value);
                    }
                }
            />
        </div>
    }
}