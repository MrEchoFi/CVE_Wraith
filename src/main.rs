/*
//Without NVD API KEY CODE:
use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Terminal,
};
use reqwest::Client;
use serde_json::Value;
use tokio::time::sleep;

#[derive(PartialEq)]
enum InputField {
    Vendor,
    Product,
    Version,
    Results,
}

struct App {
    vendor: String,
    product: String,
    version: String,
    active: InputField,
    logs: Vec<String>,
    scroll: u16,
    scrollbar_state: ScrollbarState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            vendor: String::new(),
            product: String::new(),
            version: String::new(),
            active: InputField::Vendor,
            logs: vec![],
            scroll: 0,
            scrollbar_state: ScrollbarState::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Tab => {
                            app.active = match app.active {
                                InputField::Vendor => InputField::Product,
                                InputField::Product => InputField::Version,
                                InputField::Version => InputField::Results,
                                InputField::Results => InputField::Vendor,
                            }
                        }
                        KeyCode::Enter => {
                            if app.active == InputField::Version {
                                fetch_cves(&mut app).await?;
                                app.active = InputField::Results;
                            }
                        }
                        KeyCode::Up => {
                            if app.active == InputField::Results && app.scroll > 0 {
                                app.scroll -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.active == InputField::Results {
                                app.scroll += 1;
                            }
                        }
                        KeyCode::Char(c) => match app.active {
                            InputField::Vendor => app.vendor.push(c),
                            InputField::Product => app.product.push(c),
                            InputField::Version => app.version.push(c),
                            _ => {}
                        },
                        KeyCode::Backspace => match app.active {
                            InputField::Vendor => { app.vendor.pop(); }
                            InputField::Product => { app.product.pop(); }
                            InputField::Version => { app.version.pop(); }
                            _ => {}
                        },
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::ScrollUp && app.scroll > 0 {
                        app.scroll -= 1;
                    }
                    if mouse.kind == MouseEventKind::ScrollDown {
                        app.scroll += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(5),
        ])
        .split(f.size());

    draw_inputs(f, chunks[0], app);
    draw_logs(f, chunks[1], app);
}

fn draw_inputs(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let text = format!(
        "Vendor  : {}\nProduct : {}\nVersion : {}",
        app.vendor, app.product, app.version
    );

    let block = Paragraph::new(text)
        .block(Block::default().title(">CVE Wraith_ Input").borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(block, area);

    // Visible cursor
    match app.active {
        InputField::Vendor => {
            f.set_cursor(area.x + 10 + app.vendor.len() as u16, area.y + 1);
        }
        InputField::Product => {
            f.set_cursor(area.x + 10 + app.product.len() as u16, area.y + 2);
        }
        InputField::Version => {
            f.set_cursor(area.x + 10 + app.version.len() as u16, area.y + 3);
        }
        _ => {}
    }
}

fn draw_logs(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let text = app.logs.join("\n\n");

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Results Logs").borders(Borders::ALL))
        .scroll((app.scroll, 0));

    f.render_widget(paragraph, area);

    let content_height = app.logs.len() as u16 * 4;
    app.scrollbar_state = app.scrollbar_state.content_length(content_height.into());
    app.scrollbar_state = app.scrollbar_state.position(app.scroll.into());

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight);

    f.render_stateful_widget(scrollbar, area, &mut app.scrollbar_state);
}

async fn fetch_cves(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.logs.clear();
    app.logs.push("Fetching CVEs...".into());

    let client = Client::new();

    let url = format!(
        "https://services.nvd.nist.gov/rest/json/cves/2.0?cpeName=cpe:2.3:a:{}:{}:{}:*:*:*:*:*:*:*",
        app.vendor.to_lowercase(),
        app.product.to_lowercase(),
        app.version
    );

    let response: Value = client.get(&url).send().await?.json().await?;

    if let Some(vulns) = response["vulnerabilities"].as_array() {
        if vulns.is_empty() {
            app.logs.push("No CVEs found.".into());
        } else {
            for vuln in vulns {
                let id = vuln["cve"]["id"].as_str().unwrap_or("Unknown");
                let desc = vuln["cve"]["descriptions"]
                    .get(0)
                    .and_then(|d| d["value"].as_str())
                    .unwrap_or("No description");

                app.logs.push(format!("CVE: {}\nDescription: {}", id, desc));
            }
        }
    }

    sleep(Duration::from_millis(300)).await;
    Ok(())
}
    */

//Using NVD API KEY CODE:
use std::{env, error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    //text::Span,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Terminal,
};
use reqwest::Client;
use serde_json::Value;
use tokio::time::sleep;

const ASCII_ART: &str = r#"_________ ____   _______________     __      __                 .__   __     ___ ___   
\_   ___ \\   \ /   /\_   _____/    /  \    /  \_______ _____   |__|_/  |_  /   |   \  
/    \  \/ \   Y   /  |    __)_     \   \/\/   /\_  __ \\__  \  |  |\   __\/    ~    \ 
\     \____ \     /   |        \     \        /  |  | \/ / __ \_|  | |  |  \    Y    / 
 \______  /  \___/   /_______  /______\__/\  /   |__|   (____  /|__| |__|   \___|_  /  
        \/                   \//_____/     \/                \/                   \/   
    NET_Wraith-MrEchoFi                                                                                     
"#;

#[derive(PartialEq)]
enum InputField {
    Vendor,
    Product,
    Version,
    Results,
}

struct App {
    vendor: String,
    product: String,
    version: String,
    active: InputField,
    logs: Vec<String>,
    scroll: u16,
    scrollbar_state: ScrollbarState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            vendor: String::new(),
            product: String::new(),
            version: String::new(),
            active: InputField::Vendor,
            logs: vec![],
            scroll: 0,
            scrollbar_state: ScrollbarState::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Tab => {
                            app.active = match app.active {
                                InputField::Vendor => InputField::Product,
                                InputField::Product => InputField::Version,
                                InputField::Version => InputField::Results,
                                InputField::Results => InputField::Vendor,
                            }
                        }
                        KeyCode::Enter => {
                            if app.active == InputField::Version {
                                fetch_cves(&mut app).await?;
                                app.active = InputField::Results;
                            }
                        }
                        KeyCode::Up => {
                            if app.scroll > 0 {
                                app.scroll -= 1;
                            }
                        }
                        KeyCode::Down => {
                            app.scroll += 1;
                        }
                        KeyCode::Char(c) => match app.active {
                            InputField::Vendor => app.vendor.push(c),
                            InputField::Product => app.product.push(c),
                            InputField::Version => app.version.push(c),
                            _ => {}
                        },
                        KeyCode::Backspace => match app.active {
                            InputField::Vendor => { app.vendor.pop(); }
                            InputField::Product => { app.product.pop(); }
                            InputField::Version => { app.version.pop(); }
                            _ => {}
                        },
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::ScrollUp && app.scroll > 0 {
                        app.scroll -= 1;
                    }
                    if mouse.kind == MouseEventKind::ScrollDown {
                        app.scroll += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Min(5),
        ])
        .split(f.size());

    // ASCII Banner
    let banner = Paragraph::new(ASCII_ART)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(banner, chunks[0]);

    draw_inputs(f, chunks[1], app);
    draw_logs(f, chunks[2], app);
}

fn draw_inputs(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let text = format!(
        "Vendor  : {}\nProduct : {}\nVersion : {}",
        app.vendor, app.product, app.version
    );

    let block = Paragraph::new(text)
        .block(Block::default().title("> Input").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(block, area);

    match app.active {
        InputField::Vendor => {
            f.set_cursor(area.x + 10 + app.vendor.len() as u16, area.y + 1);
        }
        InputField::Product => {
            f.set_cursor(area.x + 10 + app.product.len() as u16, area.y + 2);
        }
        InputField::Version => {
            f.set_cursor(area.x + 10 + app.version.len() as u16, area.y + 3);
        }
        _ => {}
    }
}

fn draw_logs(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let text = app.logs.join("\n\n");

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Results Logs").borders(Borders::ALL))
        .scroll((app.scroll, 0));

    f.render_widget(paragraph, area);

    let content_height = app.logs.len() as u16 * 4;
    app.scrollbar_state = app.scrollbar_state.content_length(content_height.into());
    app.scrollbar_state = app.scrollbar_state.position(app.scroll.into());

    let scrollbar =
        Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

    f.render_stateful_widget(scrollbar, area, &mut app.scrollbar_state);
}

async fn fetch_cves(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.logs.clear();
    app.logs.push("- Fetching CVEs from NVD...".into());

    let api_key = env::var("NVD_API_KEY")
        .expect("ea72e05f-9830-4dcd-8cf5-428cc18e2191");  //type the api key here if you have one, otherwise it will use the default key which has very limited rate limits

    let client = Client::new();

    let url = format!(
        "https://services.nvd.nist.gov/rest/json/cves/2.0?cpeName=cpe:2.3:a:{}:{}:{}:*:*:*:*:*:*:*",
        app.vendor.to_lowercase(),
        app.product.to_lowercase(),
        app.version
    );

    let response: Value = client
        .get(&url)
        .header("apiKey", api_key)
        .send()
        .await?
        .json()
        .await?;

    if let Some(vulns) = response["vulnerabilities"].as_array() {
        if vulns.is_empty() {
            app.logs.push(">> No CVEs found.".into());
        } else {
            for vuln in vulns {
                let id = vuln["cve"]["id"].as_str().unwrap_or("Unknown");
                let desc = vuln["cve"]["descriptions"]
                    .get(0)
                    .and_then(|d| d["value"].as_str())
                    .unwrap_or("No description");

                app.logs.push(format!("-> CVE: {}\n{}", id, desc));
            }
        }
    }

    sleep(Duration::from_millis(200)).await;
    Ok(())
}