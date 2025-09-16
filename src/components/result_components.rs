use leptos::*;
use crate::beyond_531::*;

#[component]
pub fn TrainingProgramDisplay(program: Memo<TrainingProgram>) -> impl IntoView {
    let (selected_week, set_selected_week) = create_signal(1);

    let program_data = program.get();
    let selected_week_data = create_memo(move |_| {
        program_data.weeks.iter()
            .find(|week| week.week_number == selected_week.get())
            .cloned()
    });

    view! {
        <div class="training-program-container">
            <div class="week-selector">
                <h3 class="selector-title">"Select Week"</h3>
                <div class="week-tabs">
                    <For
                        each=move || program.get().weeks.clone()
                        key=|week| week.week_number
                        children=move |week| {
                            let week_num = week.week_number;
                            let is_selected = create_memo(move |_| selected_week.get() == week_num);
                            let week_title = if week_num == 4 {
                                "W4".to_string()
                            } else {
                                format!("W{}", week_num)
                            };

                            view! {
                                <button
                                    class=move || format!("week-tab {}", if is_selected.get() { "active" } else { "" })
                                    on:click=move |_| set_selected_week.set(week_num)
                                >
                                    {week_title}
                                </button>
                            }
                        }
                    />
                </div>
            </div>

            <div class="week-display-container">
                {move || {
                    if let Some(week) = selected_week_data.get() {
                        view! {
                            <WeekDisplay week=week />
                        }.into_view()
                    } else {
                        view! {
                            <div class="no-week-selected">"Select a week to view details"</div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
pub fn WeekDisplay(week: Week) -> impl IntoView {
    let week_title = if week.week_number == 4 {
        format!("Week {} - MAX WEEK!", week.week_number)
    } else {
        format!("Week {}", week.week_number)
    };
    
    view! {
        <div class="week">
            <h2 class="week-title">{week_title}</h2>
            <div class="sessions">
                <For
                    each=move || week.sessions.clone()
                    key=|session| session.day.clone()
                    children=move |session| {
                        view! {
                            <SessionDisplay session=session />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn SessionDisplay(session: Session) -> impl IntoView {
    view! {
        <div class="session">
            <h3 class="session-day">{session.day}</h3>
            <div class="exercises">
                <For
                    each=move || session.exercises.clone()
                    key=|exercise| format!("{}_{}_{}_{}", exercise.name, exercise.sets, exercise.reps, exercise.weight)
                    children=move |exercise| {
                        view! {
                            <ExerciseDisplay exercise=exercise />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn ExerciseDisplay(exercise: Exercise) -> impl IntoView {
    let display_text = if exercise.is_amrap && exercise.sets == 1 && exercise.reps == 1 {
        format!("{}: 1+ @ {}kg ({}%)", exercise.name, exercise.weight, exercise.percentage as u32)
    } else if exercise.is_amrap {
        format!("{}: {}x{}+ @ {}kg ({}%)", exercise.name, exercise.sets, exercise.reps, exercise.weight, exercise.percentage as u32)
    } else if exercise.sets == 1 && exercise.reps == 1 {
        format!("{}: {}kg ({}%)", exercise.name, exercise.weight, exercise.percentage as u32)
    } else {
        format!("{}: {}x{} @ {}kg ({}%)", exercise.name, exercise.sets, exercise.reps, exercise.weight, exercise.percentage as u32)
    };
    
    let intensity_class = match exercise.percentage as u32 {
        65 => "intensity-65",
        70 => "intensity-65", // Same color as 65%
        75 => "intensity-75",
        80 => "intensity-warmup", // For warmup sets
        85 => "intensity-85", 
        90 => "intensity-90",
        95 => "intensity-90", // Same color as 90%
        100 => "intensity-100",
        105 => "intensity-105",
        _ => "intensity-default",
    };
    
    view! {
        <div class=format!("exercise {}", intensity_class)>{display_text}</div>
    }
}