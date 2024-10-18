fn main() {
    //Option - системное перечисление с вариантами, доступными в области
    //enum Option<T> {
    //    None,
    //    Some(T),
    //}
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    

    //Новый тип перечисления
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32), 
    }
    impl Message {
        fn call(&self) {
            // Какой-то метод
        }
    }
    let m = Message::Write(String::from("Hello"));
    m.call();
}