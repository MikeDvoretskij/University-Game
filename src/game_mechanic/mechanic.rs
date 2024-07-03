use std::collections::HashMap;

fn game() {
    let mut player = Player::new("Миша".to_string(), 0.0);

    let mut h1: HashMap<i8, String> = HashMap::new();
    h1.insert(1, "Венера".to_string());
    h1.insert(2, "Земля".to_string());
    h1.insert(3, "Юпитер".to_string());
    h1.insert(4, "Марс".to_string());

    let mut h2: HashMap<i8, String> = HashMap::new();
    h2.insert(1, "Фёдор Достоевский".to_string());
    h2.insert(2, "Лев Толстой".to_string());
    h2.insert(3, "Александр Пушкин".to_string());
    h2.insert(4, "Николай Гоголь".to_string());

    let mut h3: HashMap<i8, String> = HashMap::new();
    h3.insert(1, "Франция".to_string());
    h3.insert(2, "Германия".to_string());
    h3.insert(3, "Норвегия".to_string());
    h3.insert(4, "Испания".to_string());

    let mut h4: HashMap<i8, String> = HashMap::new();
    h4.insert(1, "Водород".to_string());
    h4.insert(2, "Кислород".to_string());
    h4.insert(3, "Азот".to_string());
    h4.insert(4, "Углерод".to_string());

    let q_1 = Question_One{
        question: "Какая планета является самой большой в Солнечной системе?".to_string(),
        answer: "Юпитер".to_string(),
        answers: h1
    };

    let q_2 = Question_One{
        question: "Кто написал роман 'Война и мир'".to_string(),
        answer: "Лев Толстой".to_string(),
        answers: h2
    };

    let q_3 = Question_One{
        question: "Какая из этих стран не является членом Европейского союза?".to_string(),
        answer: "Норвегия".to_string(),
        answers: h3
    };

    let q_4 = Question_One{
        question: "Какой элемент имеет химический символ 'O'?".to_string(),
        answer: "Кислород".to_string(),
        answers: h4
    };

    let vec_q = vec![q_1, q_2, q_3, q_4];

    for q in &vec_q {
        println!("----------{:?}----------", q.question);
        for answer in &q.answers {
            println!("{}. {}", answer.0, answer.1)
        }
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Something went wrong");
        let user_input = user_input;

        match user_input.trim().parse::<i8>() {
            Ok(key) => {
                let answer = q.answers.get(&key).unwrap().to_string();
                if answer == q.answer {
                    player.score = player.score + 1.0
                }
            }
            Err(_) => {
                println!("Something went wrong")
            }
        }
    }

    println!("Name: {}, Score: {}", player.user_name, player.score)
}