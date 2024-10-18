//Используемый код - только для изучения
//Определение emun
enum IpAddrKind {
    V4,
    V6
}

//Определение структуры
struct IpAddr {
    king: IpAddrKind,
    address: String
}

//Можно определить более локанично - выше перечисленные структуры и перечисления
enum IpAddrL {
    V4(String),
    V6(String),
}
//Еще один варинт определения, с указанием более точных типов
enum IpAddrR {
    V4(u8, u8, u8, u8), //u8 - Значение от 0 до 255
    V6(String),
}
//Определение из стандартной библиотеки
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddrS {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    //Тренировка
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //Использование определенной функции *route
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    //Новый способ использования
    let home = IpAddr {
        king: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    //Для примера использовал ранее объявленную переменную
    //но, для перечисления думаю нет в этом какого-то смысла
    let loopback = IpAddr {
        king: six,
        address: String::from("::1"),
    };
    //Использование более локаничного варианта кода
    let home_l = IpAddrL::V4(String::from("192.168.0.1"));
    let loopback_l = IpAddrL::V6(String::from("::1"));

    let home_r = IpAddrR::V4(127, 0, 0, 1);
    let loopback_r = IpAddrR::V6(String::from("::1"));

    println!("Hello, world!");
}

//Определение функции для использования enum
fn route(ip_king: IpAddrKind) {}