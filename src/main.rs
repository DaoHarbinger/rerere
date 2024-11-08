fn main() {
    const SIZE: usize = 5; // Висота верхньої половини ромба (зірок у центральному ряду)

    // Верхня частина ромба
    for i in 0..=SIZE {
        let spaces = SIZE - i; // Кількість пробілів перед зірками
        let stars = 2 * i + 1; // Кількість зірок у рядку
        print!("{:width$}", "", width = spaces); // Друк пробілів
        print!("{:*<width$}", "", width = stars); // Друк зірок
        println!();
    }

    // Нижня частина ромба
    for i in (0..SIZE).rev() {
        let spaces = SIZE - i; // Кількість пробілів перед зірками
        let stars = 2 * i + 1; // Кількість зірок у рядку
        print!("{:width$}", "", width = spaces); // Друк пробілів
        print!("{:*<width$}", "", width = stars); // Друк зірок
        println!();
    }
}
