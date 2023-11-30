use tokio::process::Command;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table, Color};

pub async fn print_table(installed: &Vec<(usize, String, std::time::Duration)>) {
    let table_rows: Vec<_> = installed
        .iter()
        .map(|(i, font_name, duration)| vec![
            (i + 1).to_string().cell().justify(Justify::Center).foreground_color(Some(Color::Yellow)),
            font_name.cell().foreground_color(Some(Color::Yellow)),
            format!("{:?}", duration).cell().justify(Justify::Center).foreground_color(Some(Color::Yellow)),
        ])
        .collect();

    let table = table_rows
        .table()
        .title(vec![
            "Index".cell().bold(true).justify(Justify::Center).foreground_color(Some(Color::Yellow)),
            "Font Name".cell().bold(true).foreground_color(Some(Color::Yellow)),
            "Duration".cell().bold(true).justify(Justify::Center).foreground_color(Some(Color::Yellow)),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}

#[allow(dead_code)]
pub async fn update_font_cache() -> Result<(), std::io::Error> {
    let output = Command::new("fc-cache")
        .arg("-f")
        .arg("-v")
        .output()
        .await?;

    if !output.status.success() {
        eprintln!("fc-cache failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
