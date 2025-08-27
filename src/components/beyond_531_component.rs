use leptos::*;
use crate::beyond_531::*;
use crate::components::{InputField, TrainingProgramDisplay};

#[component]
pub fn Beyond531Calculator() -> impl IntoView {
    let (front_squat_1rm, set_front_squat_1rm) = create_signal(100.0);
    let (deadlift_1rm, set_deadlift_1rm) = create_signal(120.0);
    let (bench_press_1rm, set_bench_press_1rm) = create_signal(80.0);
    
    let training_program = create_memo(move |_| {
        let one_rep_max = OneRepMax {
            front_squat: front_squat_1rm.get(),
            deadlift: deadlift_1rm.get(),
            bench_press: bench_press_1rm.get(),
        };
        Beyond531Calculator::calculate_program(&one_rep_max)
    });
    
    let reset_calculator = move |_| {
        set_front_squat_1rm.set(100.0);
        set_deadlift_1rm.set(120.0);
        set_bench_press_1rm.set(80.0);
    };
    
    view! {
        <div class="calculator-container">
            <div class="calculator-header">
                <h1>"Beyond 531 Training Calculator"</h1>
                <div class="calculator-info">
                    <div class="program-info">
                        "4-week progressive strength program"
                    </div>
                    <button class="reset-button" on:click=reset_calculator>
                        "Reset"
                    </button>
                </div>
            </div>
            
            <div class="input-section">
                <h2>"Enter your 1 Rep Max (1RM) in kilograms"</h2>
                <div class="input-grid">
                    <InputField
                        label="Front Squat 1RM (kg)".to_string()
                        value=front_squat_1rm
                        on_change=set_front_squat_1rm
                        step=2.5
                        min=10.0
                    />
                    
                    <InputField
                        label="Deadlift 1RM (kg)".to_string()
                        value=deadlift_1rm
                        on_change=set_deadlift_1rm
                        step=2.5
                        min=10.0
                    />
                    
                    <InputField
                        label="Bench Press 1RM (kg)".to_string()
                        value=bench_press_1rm
                        on_change=set_bench_press_1rm
                        step=2.5
                        min=10.0
                    />
                </div>
            </div>
            
            <div class="program-description">
                <h3>"Program Overview"</h3>
                <ul>
                    <li><strong>"Week 1:"</strong> " Moderate intensity - 3-5x5 @ 75% both sessions"</li>
                    <li><strong>"Week 2:"</strong> " Increased intensity - Friday moves to 3x5 @ 85%"</li>
                    <li><strong>"Week 3:"</strong> " High intensity - Friday becomes 3x3 @ 90%"</li>
                    <li><strong>"Week 4:"</strong> " MAX WEEK - Test your limits up to 105%!"</li>
                </ul>
                <p>"Monday sessions are always 3-5x5 @ 75% to maintain consistent volume."</p>
            </div>
            
            <div class="results-display">
                <TrainingProgramDisplay program=training_program />
            </div>
        </div>
    }
}