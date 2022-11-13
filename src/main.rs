use std::{fs, process::ExitCode};

use anyhow::{anyhow, bail, Result};
use scraper::{Html, Selector};
use ureq::AgentBuilder;

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e:?}");
            ExitCode::FAILURE
        }
    }
}

fn try_main() -> Result<()> {
    let agent = AgentBuilder::new()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:105.0) Gecko/20100101 Firefox/105.0")
        .build();

    let (username, password) = read_creds()?;

    agent
        .post("http://mahasiswa.stmik.banisaleh.ac.id")
        .send_form(&[("username", &username), ("password", &password)])?;

    let text = agent
        .get("http://mahasiswa.stmik.banisaleh.ac.id/Jadwal")
        .call()?
        .into_string()?;

    // let text = fs::read_to_string("jadwal.html")?;

    eprintln!("[+] Parsing document..");
    let dom = Html::parse_document(&text);

    print_biodata(&dom)?;

    Ok(())
}

fn print_biodata(dom: &Html) -> Result<()> {
    let biodata = {
        let selector = Selector::parse("dl.row").unwrap();

        let biodata_container = dom
            .select(&selector)
            .next()
            .ok_or_else(|| anyhow!("Failed to login!"))?;

        let data_type_selector = Selector::parse("dd.col-sm-4").unwrap();
        let data_type = biodata_container.select(&data_type_selector);

        let values_selector = Selector::parse("dd.col-sm-8").unwrap();
        let values = biodata_container.select(&values_selector);

        data_type.zip(values).collect::<Vec<_>>()
    };

    let longest_data_type = biodata
        .iter()
        .max_by(|(a, _), (b, _)| a.text().count().cmp(&b.text().count()))
        .map(|(data_type, _)| data_type.text().map(|a| a.len()).max())
        .unwrap_or(Some(20))
        .unwrap();

    println!("[+] Logged in as: ");

    for (data, value) in biodata {
        println!(
            "{:longest_data_type$}{}",
            data.text().collect::<String>(),
            value.text().collect::<String>()
        );
    }

    Ok(())
}

fn read_creds() -> Result<(String, String)> {
    let content = fs::read_to_string("./creds.env")?;
    let (mut username, mut password) = (String::new(), String::new());

    for line in content.lines() {
        if line.starts_with("USERNAME=") {
            username = line
                .split('=')
                .last()
                .ok_or_else(|| anyhow!("Bad format!"))?
                .into();
        } else if line.starts_with("PASSWORD=") {
            password = line
                .split('=')
                .last()
                .ok_or_else(|| anyhow!("Bad format!"))?
                .split(',')
                .filter_map(|val| val.parse::<u8>().ok())
                .map(|val| (val ^ b'A') as char)
                .collect();
        }
    }

    if username.is_empty() {
        bail!("Failed to read username!");
    }

    if password.is_empty() {
        bail!("Failed to read password!");
    }

    Ok((username, password))
}
