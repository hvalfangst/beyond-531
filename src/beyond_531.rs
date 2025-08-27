#[derive(Debug, Clone, PartialEq)]
pub struct OneRepMax {
    pub front_squat: f64,
    pub deadlift: f64,
    pub bench_press: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub name: String,
    pub sets: u32,
    pub reps: u32,
    pub weight: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub day: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Week {
    pub week_number: u32,
    pub sessions: Vec<Session>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingProgram {
    pub weeks: Vec<Week>,
}

pub struct Beyond531Calculator;

impl Beyond531Calculator {
    pub fn calculate_program(one_rep_max: &OneRepMax) -> TrainingProgram {
        let mut weeks = Vec::new();
        
        for week_number in 1..=4 {
            let mut sessions = Vec::new();
            
            // Monday session (always 3-5x5 at 75%)
            let monday_exercises = vec![
                Exercise {
                    name: "Front Squat".to_string(),
                    sets: 5,
                    reps: 5,
                    weight: Self::round_to_2_5(one_rep_max.front_squat * 0.75),
                    percentage: 75.0,
                },
                Exercise {
                    name: "Deadlift".to_string(),
                    sets: 3,
                    reps: 5,
                    weight: Self::round_to_2_5(one_rep_max.deadlift * 0.75),
                    percentage: 75.0,
                },
                Exercise {
                    name: "Bench Press".to_string(),
                    sets: 5,
                    reps: 5,
                    weight: Self::round_to_2_5(one_rep_max.bench_press * 0.75),
                    percentage: 75.0,
                },
            ];
            
            sessions.push(Session {
                day: "Monday".to_string(),
                exercises: monday_exercises,
            });
            
            // Friday session (varies by week)
            let friday_exercises = match week_number {
                1 => {
                    // Week 1: 5x5 at 75%
                    vec![
                        Exercise {
                            name: "Front Squat".to_string(),
                            sets: 5,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.front_squat * 0.75),
                            percentage: 75.0,
                        },
                        Exercise {
                            name: "Deadlift".to_string(),
                            sets: 3,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.deadlift * 0.75),
                            percentage: 75.0,
                        },
                        Exercise {
                            name: "Bench Press".to_string(),
                            sets: 5,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.bench_press * 0.75),
                            percentage: 75.0,
                        },
                    ]
                },
                2 => {
                    // Week 2: 5x5 at 85%
                    vec![
                        Exercise {
                            name: "Front Squat".to_string(),
                            sets: 5,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.front_squat * 0.85),
                            percentage: 85.0,
                        },
                        Exercise {
                            name: "Deadlift".to_string(),
                            sets: 3,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.deadlift * 0.85),
                            percentage: 85.0,
                        },
                        Exercise {
                            name: "Bench Press".to_string(),
                            sets: 5,
                            reps: 5,
                            weight: Self::round_to_2_5(one_rep_max.bench_press * 0.85),
                            percentage: 85.0,
                        },
                    ]
                },
                3 => {
                    // Week 3: 3x3 at 90%
                    vec![
                        Exercise {
                            name: "Front Squat".to_string(),
                            sets: 3,
                            reps: 3,
                            weight: Self::round_to_2_5(one_rep_max.front_squat * 0.90),
                            percentage: 90.0,
                        },
                        Exercise {
                            name: "Deadlift".to_string(),
                            sets: 3,
                            reps: 3,
                            weight: Self::round_to_2_5(one_rep_max.deadlift * 0.90),
                            percentage: 90.0,
                        },
                        Exercise {
                            name: "Bench Press".to_string(),
                            sets: 3,
                            reps: 3,
                            weight: Self::round_to_2_5(one_rep_max.bench_press * 0.90),
                            percentage: 90.0,
                        },
                    ]
                },
                4 => {
                    // Week 4: Max week - 3@65%, 1@80%, 1@90%, 1@100%, 1@105%
                    let mut exercises = Vec::new();
                    
                    for (exercise_name, one_rm) in [
                        ("Front Squat", one_rep_max.front_squat),
                        ("Deadlift", one_rep_max.deadlift),
                        ("Bench Press", one_rep_max.bench_press),
                    ] {
                        exercises.extend([
                            Exercise {
                                name: format!("{} - Single", exercise_name),
                                sets: 1,
                                reps: 1,
                                weight: Self::round_to_2_5(one_rm * 0.80),
                                percentage: 80.0,
                            },
                            Exercise {
                                name: format!("{} - Single", exercise_name),
                                sets: 1,
                                reps: 1,
                                weight: Self::round_to_2_5(one_rm * 0.90),
                                percentage: 90.0,
                            },
                            Exercise {
                                name: format!("{} - Max", exercise_name),
                                sets: 1,
                                reps: 1,
                                weight: Self::round_to_2_5(one_rm * 1.00),
                                percentage: 100.0,
                            },
                            Exercise {
                                name: format!("{} - BEYOND!", exercise_name),
                                sets: 1,
                                reps: 1,
                                weight: Self::round_to_2_5(one_rm * 1.05),
                                percentage: 105.0,
                            },
                        ]);
                    }
                    exercises
                },
                _ => unreachable!(),
            };
            
            sessions.push(Session {
                day: "Friday".to_string(),
                exercises: friday_exercises,
            });
            
            weeks.push(Week {
                week_number,
                sessions,
            });
        }
        
        TrainingProgram { weeks }
    }
    
    fn round_to_2_5(weight: f64) -> f64 {
        (weight / 2.5).round() * 2.5
    }
}