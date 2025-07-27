use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use std::io;
use std::io::Write;
use std::process::exit;
use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode};

mod network;
mod parser;
mod settings;

#[tokio::main]
async fn main() {

    // 0. App settings
    let (settings, config_file_message) = settings::load_setting();
    println!("{}", config_file_message);
    println!("{}", settings.to_string());

    // 1. Fetch string data from url
    println!("Выполняется подключение к {}", settings.url);
    let result_text_body = network::fetch_raw_data(settings.url).await;
    if result_text_body.is_err() {
        println!("{}", result_text_body.unwrap_err());
        finish(15).await;
        return;
    }
    let text_body = result_text_body.unwrap();
    println!("Получен ответ размером {} байт", text_body.as_bytes().len());

    // 2. Parse data into array of tuples [(ProtoType, Url)]
    let all_servers = parser::parse_body(&text_body);
    println!("Всего серверов найдено - {}", all_servers.len());

    // 3. Filter array according to application settings
    let urls = all_servers
        .iter()
        .filter(|tuple| settings.proto_types.contains(&tuple.0))
        .map(|tuple| &tuple.1);
    let urls_count = urls.clone().count();
    println!("Из них удовлетворяют настройкам - {}", urls_count);

    // 4. Save to files
    if urls_count > 0 {
        // Magic values:
        const F_CREATED: isize = 2;
        const F_UPDATED: isize = 1;
        const F_FAILED: isize = 0;

        let summary = urls
            .map(async |url| {
                println!("Start saving from {url}");
                let res = network::save_to_file(url).await;
                match res {
                    Ok(tuple) => {
                        if tuple.1 {
                            println!("Обновлено: {}", tuple.0);
                            F_UPDATED
                        } else {
                            println!("Создано: {}", tuple.0);
                            F_CREATED
                        }
                    }
                    Err(e) => {
                        println!("Ошибка: {e}");
                        F_FAILED
                    }
                }
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;

        let created = summary.iter().filter(|&&x| x == F_CREATED).count();
        let updated = summary.iter().filter(|&&x| x == F_UPDATED).count();
        println!("Итого: создано - {created}, обновлено - {updated}");
    }

    // 5. The final countdown
    finish(15).await;
}

async fn finish(seconds: u8) {
    print!("Выполнение программы будет завершено через {seconds} секунд ");
    print!("Нажмите любую клавишу, чтобы отменить завершение ");

    let join = tokio::task::spawn(async move {
        let device_state = DeviceState::new();
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if !keys.is_empty() {
                break;
            }
        }
    });

    for _ in 0..seconds {
        async_std::task::sleep(Duration::from_millis(999)).await;
        if join.is_finished() {
            break;
        }

        print!(".");
        io::stdout().flush().expect("TODO: panic message");
    }

    // infinite loop
    if join.is_finished() {
        println!("\nЗавершение отменено пользователем");
        let markers = [
            "-",
            "\\",
            "|",
            "/",
            "- н       c      ч     з",
            "\\ на      ct     чт    за",
            "| наж     ctr    что   зак",
            "/ нажм    ctrl   чтоб  закр",
            "- нажми   ctrl+  чтобы закры",
            "\\ нажмит  ctrl+c чтобы закрыт",
            "| нажмите ctrl+c чтобы закрыть",
            "/ нажмите ctrl+c чтобы закрыть",
            "- нажмите ctrl+c чтобы закрыть",
            "\\ нажмите ctrl+c чтобы закрыть",
            "| нажмите ctrl+c чтобы закрыть",
            "/ нажмите ctrl+c чтобы закрыть",
            "- нажмите ctrl+c чтобы закрыть",
            "\\  ажмите  trl+c  тобы  акрыть",
            "|   жмите   rl+c   обы   крыть",
            "/    мите    l+c    бы    рыть",
            "-     ите     +c     ы     ыть",
            "\\      те      c            ть",
            "|       е                    ь",
            "/                              ",
            "-",
            "\\",
            "|",
            "/",
            "-",
            "\\",
            "|",
            "/",
            "-",
            "\\",
            "|",
            "/",
            "-",
            "\\",
            "|",
            "/",
            "-",
            "\\",
            "|",
            "/",
            "-",
            "\\",
            "|",
            "/",
        ];
        let mut i = 0;
        loop {
            async_std::task::sleep(Duration::from_millis(200)).await;
            print!("\r{}", markers[i]);
            io::stdout().flush().expect("TODO: panic message");
            i = (i + 1 ) % markers.len();
        }
    }

    exit(0);
}
