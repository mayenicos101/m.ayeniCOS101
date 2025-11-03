use std::io;

fn main() {
    let mut name = String::new();
    let mut score_1 = String::new();
    let mut score_2 = String::new();
    let mut score_3 = String::new();

    println!("\nWhat is your name?");
    io::stdin().read_line(&mut name).expect("Name input is invalid");

    println!("Input your first test score:");
    io::stdin().read_line(&mut score_1).expect("Not a valid string");
    let score_1:f32 = score_1.trim().parse().expect("Score is invalid");

     if score_1 < 0.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }

    if score_1 > 100.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }

    println!("Input your second test score:");
    io::stdin().read_line(&mut score_2).expect("Not a valid string");
    let score_2:f32 = score_2.trim().parse().expect("Score is invalid");

   if score_2 < 0.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }

    if score_2 > 100.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }


    println!("Input your third test score:");
    io::stdin().read_line(&mut score_3).expect("Not a valid string");
    let score_3:f32 = score_3.trim().parse().expect("Score is invalid");

    if score_3 < 0.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }

    if score_3 > 100.0 {
        println!("A score must be in between 0 and 100 inclusive");
        return;
    }

    let average:f32 = (score_1 + score_2 + score_3) / 3.0;
    let grade:char;

    if average >= 70.0 && average <= 100.0 {
        grade = 'A';
    } else if average >= 60.0 && average <= 69.0 {
        grade = 'B';
    } else if average >= 50.0 && average <= 59.0 {
        grade = 'C';
    } else if average >= 45.0 && average <= 49.0 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("\n{}
        \nAverage:{:.2}
        \nGrade: {}", name, average, grade);
}