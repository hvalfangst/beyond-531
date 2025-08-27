use leptos::*;
use crate::beyond_531::{OneRepMax, Beyond531Calculator as Calculator};
use crate::components::{InputField, TrainingProgramDisplay};

#[component]
pub fn Beyond531Calculator() -> impl IntoView {
    let (front_squat_1rm, set_front_squat_1rm) = create_signal(0.0);
    let (deadlift_1rm, set_deadlift_1rm) = create_signal(0.0);
    let (bench_press_1rm, set_bench_press_1rm) = create_signal(0.0);
    let (program_generated, set_program_generated) = create_signal(false);
    
    let has_valid_inputs = create_memo(move |_| {
        front_squat_1rm.get() > 0.0 && deadlift_1rm.get() > 0.0 && bench_press_1rm.get() > 0.0
    });
    
    let training_program = create_memo(move |_| {
        if program_generated.get() && has_valid_inputs.get() {
            let one_rep_max = OneRepMax {
                front_squat: front_squat_1rm.get(),
                deadlift: deadlift_1rm.get(),
                bench_press: bench_press_1rm.get(),
            };
            leptos::logging::log!("Recalculating program with: squat={}, deadlift={}, bench={}", 
                one_rep_max.front_squat, one_rep_max.deadlift, one_rep_max.bench_press);
            Some(Calculator::calculate_program(&one_rep_max))
        } else {
            None
        }
    });
    
    let generate_program = move |_| {
        set_program_generated.set(true);
    };
    
    let reset_calculator = move |_| {
        set_front_squat_1rm.set(0.0);
        set_deadlift_1rm.set(0.0);
        set_bench_press_1rm.set(0.0);
        set_program_generated.set(false);
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
                {move || {
                    if !has_valid_inputs.get() {
                        view! {
                            <div class="input-prompt">
                                <p class="instruction">"Please enter your 1RM for all three exercises:"</p>
                            </div>
                        }
                    } else if !program_generated.get() {
                        view! {
                            <div class="input-complete">
                                <p class="success">"✓ All 1RM values entered!"</p>
                                <button class="generate-button" on:click=generate_program>
                                    "Generate Training Program"
                                </button>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="input-complete">
                                <p class="success">"✓ Your training program is ready below!"</p>
                            </div>
                        }
                    }
                }}
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
            
            {move || {
                if program_generated.get() && has_valid_inputs.get() {
                    view! {
                        <div>
                            <div class="program-description">
                                <h3>"Program Overview"</h3>
                                <ul>
                                    <li><strong>"Week 1:"</strong> " Moderate intensity - Friday moves to 5x3 @ 80%"</li>
                                    <li><strong>"Week 2:"</strong> " Increased intensity - Friday moves to 5x3 @ 85%"</li>
                                    <li><strong>"Week 3:"</strong> " High intensity - Friday becomes 3x3 @ 90% (DL: 1x3)"</li>
                                    <li><strong>"Week 4:"</strong> " MAX WEEK - Test your limits up to 105%!"</li>
                                </ul>
                                <p>"Monday sessions: Weeks 1&4 @ 65%, Weeks 2&3 @ 75% to manage CNS fatigue."</p>
                            </div>
                            
                            <div class="results-display">
                                <TrainingProgramDisplay program=create_memo(move |_| training_program.get().unwrap()) />
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div></div>
                    }
                }
            }}
        </div>
    }
}