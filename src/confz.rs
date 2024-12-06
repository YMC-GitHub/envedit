struct Config {
    name: String,
    value: i32,
}
// 定义一个简单的错误类型，这里只是占位示例，可根据实际情况完善
#[derive(Debug)]
struct Error;

fn get_config() -> Config {
    Config {
        name: String::from("default_name"),
        value: 42,
    }
}

fn get_config_option() -> Option<Config> {
    // 模拟根据命令行参数进行判断，这里简单用一个随机条件示例
    if std::env::args().any(|arg| arg == "--enable-config") {
        Some(Config {
            name: String::from("option_name"),
            value: 100,
        })
    } else {
        None
    }
}

fn get_config_result() -> Result<Config, Error> {
    // 模拟解析命令行参数出现错误的情况，这里简单返回错误
    Err(Error)
    // 如果解析成功，可以类似下面这样返回
    // Ok(Config {
    //     name: String::from("result_name"),
    //     value: 200,
    // })
}

fn main() {
    let config = get_config();
    println!("Config: {:?}", config);

    if let Some(config_option) = get_config_option() {
        println!("Config Option: {:?}", config_option);
    }

    match get_config_result() {
        Ok(config_result) => println!("Config Result: {:?}", config_result),
        Err(e) => println!("Error getting config result: {:?}", e),
    }
}
