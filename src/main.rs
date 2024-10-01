use reqwest::blocking::{Client};
use reqwest::{Url};
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path};

// Функция для скачивания отдельного ресурса (файла)
fn download_resource(client: &Client, url: &Url, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Получить ответ на HTTP-запрос GET
    let resource_response = client.get(url.as_str()).send()?.bytes()?;

    // Создать путь к скачанному файлу, используя последнюю часть URL
    let resource_path = Path::new(output_dir).join(url.path().split('/').last().unwrap());

    // Создать все необходимые папки для сохранения файла
    fs::create_dir_all(resource_path.parent().unwrap())?;

    // Сохранить скачанный ресурс в файл
    fs::write(resource_path, resource_response)?;

    Ok(())
}

// Функция для скачивания сайта целиком
fn download(url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Создать новый HTTP-клиент
    let client = Client::new();

    // Получить HTML-код сайта по указанному URL
    let response = client.get(url).send()?.text()?; // Исправленная строка

    // Парсить полученный HTML-код
    let parsed = Html::parse_document(&response);

    // Создать селектор для поиска ссылок (link), скриптов (script) и изображений (img)
    let selector = Selector::parse("link[href], script[src], img[src]").unwrap();

    // Перебрать все найденные элементы HTML-кода
    for element in parsed.select(&selector) {
        // Получить значение атрибута href или src из элемента
        let resource_url = element.value().attr("href").or_else(|| element.value().attr("src"));

        // Если значение атрибута найдено
        if let Some(resource_url) = resource_url {
            // Попытаться создать полный URL для скачиваемого ресурса
            let full_url = Url::parse(resource_url).unwrap_or_else(|_| {
                Url::parse(url).unwrap().join(resource_url).unwrap()
            });

            // Скачать ресурс с полученного URL
            if let Err(err) = download_resource(&client, &full_url, output_dir) {
                eprintln!("Не удалось скачать ресурс {}: {}", full_url, err);
            }
        }
    }

    // Сохранить полученный HTML-код сайта в файл index.html
    let html_path = Path::new(output_dir).join("index.html");
    fs::write(html_path, response)?;

    Ok(())
}

fn main() {
    // URL-адрес сайта для скачивания
    let url = "https://sanstv.ru/color";

    // Папка для сохранения скачанного сайта
    let output_dir = "downloaded_site";

    // Выполнить скачивание сайта и обработать возможные ошибки
    match download(url, output_dir) {
        Ok(_) => println!("Скачивание завершено!"),
        Err(err) => eprintln!("Ошибка: {}", err),
    }
}