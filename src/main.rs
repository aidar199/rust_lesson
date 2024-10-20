//Компактное управление потоком выполнения с if let
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => print!("The maximum is configured to be {max}"),
        _ => (),
    }
    //Второй варинт этого кода
    if let Some(max) = config_max {
        println!("The maximum is configured to max {max}");
    }
}