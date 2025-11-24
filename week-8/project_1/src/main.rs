use std::io;

fn main() {
    let aps_1_2 = vec![
        ("Office Administrator", "Intern"),
        ("Academic", "-"),
        ("Lawyer", "Paralegal"),
        ("Teacher", "Placement"),
    ];

    let aps_3_4 = vec![
        ("Office Administrator", "Administrator"),
        ("Academic", "Research Assistant"),
        ("Lawyer", "Junior Associate"),
        ("Teacher", "Classroom Teacher"),
    ];

    let aps_6_8 = vec![
        ("Office Administrator", "Senior Administrator"),
        ("Academic", "PhD Candidate"),
        ("Lawyer", "Associate"),
        ("Teacher", "Snr Teacher"),
    ];

    let el1_9_10 = vec![
        ("Office Administrator", "Office Manager"),
        ("Academic", "Post-Doc Researcher"),
        ("Lawyer", "Senior Associate 1-2"),
        ("Teacher", "Leading Teacher"),
    ];

    let el2_11_13 = vec![
        ("Office Administrator", "Director"),
        ("Academic", "Senior Lecturer"),
        ("Lawyer", "Senior Associate 3-4"),
        ("Teacher", "Deputy Principal"),
    ];

    let ses = vec![
        ("Office Administrator", "CEO"),
        ("Academic", "Dean"),
        ("Lawyer", "Partner"),
        ("Teacher", "Principal"),
    ];

    let all_levels = vec![
        ("APS 1-2", 1, 2, aps_1_2),
        ("APS 3-4", 3, 4, aps_3_4),
        ("APS 6-8", 6, 8, aps_6_8),
        ("EL1 9-10", 9, 10, el1_9_10),
        ("EL2 11-13", 11, 13, el2_11_13),
        ("SES", 14, usize::MAX, ses),
    ];

    let profession = loop {
        let p = input("Enter profession: (Office Administrator, Academic, Lawyer, Teacher)");
        let p_norm = p.to_lowercase();

        match p_norm.as_str() {
            "office administrator" => break "Office Administrator".to_string(),
            "academic" => break "Academic".to_string(),
            "lawyer" => break "Lawyer".to_string(),
            "teacher" => break "Teacher".to_string(),
            _ => println!("Invalid profession. Try again.\n"),
        }
    };

    let years: usize = loop {
        let s = input("Enter years of work experience:");
        match s.parse() {
            Ok(n) => break n,
            Err(_) => println!("Please enter a valid non-negative integer.\n"),
        }
    };

    let title = loop {
        let mut titles_for_prof: Vec<&str> = Vec::new();
        for (_, _, _, list) in &all_levels {
            for (p, t) in list {
                if *p == profession {
                    titles_for_prof.push(*t);
                }
            }
        }

        println!("\nAvailable titles for {}:", profession);
        for (i, t) in titles_for_prof.iter().enumerate() {
            println!("  {}. {}", i + 1, t);
        }

        let s = input("Choose a title by number:");
        match s.parse::<usize>() {
            Ok(n) if n > 0 && n <= titles_for_prof.len() => break titles_for_prof[n - 1].to_string(),
            _ => println!("Invalid selection. Try again.\n"),
        }
    };

    // Determine APS level based on profession, years, and title
    let aps_level = all_levels.iter().find(|(_, min, max, list)| {
        years >= *min && years <= *max && list.iter().any(|(p, t)| *p == profession && *t == title)
    });

    match aps_level {
        Some((lvl_name, _, _, _)) => println!("\nAPS Level determined: {}", lvl_name),
        None => {
            println!("\nNo APS level matches the combination of profession, years, and title.");
            return;
        }
    }

    println!("\n---------------------");
    println!("Final Result:");
    println!("Profession : {}", profession);
    println!("Job Title  : {}", title);
    println!("APS Level  : {}", aps_level.unwrap().0);
    println!("---------------------");
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}