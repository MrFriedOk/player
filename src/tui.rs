use crate::player;
use crate::track::Track;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use image::ImageReader;
use ratatui::layout::{Constraint, Layout, Size};
use ratatui::style::{Color, Modifier};
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListItem, ListState, Padding, Tabs, Widget};
use ratatui::widgets::{GraphType, Paragraph};
use ratatui::{DefaultTerminal, Frame, symbols};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use ratatui_image::Image;
use ratatui_image::picker::Picker;
use std::io::{self, Cursor};
use std::path::PathBuf;

pub struct App {
    exit: bool,
    active_screen: usize,
    tracks: Vec<Track>,
    select: usize,
    player: player::Player,
    query: String,
    loop_one: bool,
    searching: bool,
}

impl App {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            exit: false,
            active_screen: 0,
            tracks,
            select: 0,
            player: player::Player::new(),
            query: String::new(),
            loop_one: false,
            searching: false,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        //println!("{:?}", &self.tracks);
        while !self.exit {
            let mut state = ListState::default().with_selected(Some(self.select));
            terminal.draw(|frame| self.render(frame, &mut state))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, state: &mut ListState) {
        let v_layout = Layout::vertical([Constraint::Percentage(7), Constraint::Percentage(93)]);
        let h_layout: Layout =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(77)]);
        let [tabs, main] = frame.area().layout(&v_layout);
        let [left, right] = main.layout(&h_layout);

        self.render_tabs(frame, tabs);

        if self.active_screen == 1 {
            self.render_songs(frame, left, state);
        } else {
            self.render_home(frame, main);
        }
    }

    fn render_home(&mut self, frame: &mut Frame, area: Rect) {
        let v_layout = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ]);

        let [search_area, main, playback_area] = area.layout(&v_layout);

        let h_layout = Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(77)]);

        let [playlists, visual] = main.layout(&h_layout);

        let visual_v_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [visualizer, image_container] = visual.layout(&visual_v_layout);

        let search = Paragraph::new(&*self.query).block(Block::bordered().title("search")); //Block::bordered().title("search");
        let playback = Block::bordered().title("playback");
        frame.render_widget(playback, playback_area);
        frame.render_widget(search, search_area);

        if let Some(ref bytes) = self.player.image {
            self.render_image(frame, image_container, bytes).unwrap();
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                //KeyCode::Char(char) => self.exit(),
                KeyCode::Esc => self.exit(),
                KeyCode::Tab => self.active_screen = (self.active_screen + 1) % 2,
                KeyCode::Down => {
                    if self.active_screen == 1 {
                        if self.select == self.tracks.len() + 1 {
                            self.select = 0
                        } else {
                            self.select += 1
                        }
                    }
                } //self.select += 1,
                KeyCode::Up => {
                    if self.active_screen == 1 {
                        if self.select == 0 {
                            self.select = self.tracks.len() - 1
                        } else {
                            self.select -= 1
                        }
                    }
                }
                KeyCode::Enter => {
                    if self.active_screen == 1 {
                        self.player.play(&mut self.tracks[self.select])?
                    }
                }
                KeyCode::Backspace => {
                    if self.active_screen == 0 {
                        self.query.pop();
                    }
                }
                KeyCode::Char(char) => {
                    if self.active_screen == 1 {
                        match char {
                            ' ' => self.player.toggle(),
                            _ => (),
                        }
                    } else {
                        if self.searching {
                            self.query.push(char);
                        } else {
                            match char {
                                '/' => self.searching = true,
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let block: Block = Block::bordered();
        frame.render_widget(
            Tabs::new(["Home", "List"])
                .select(self.active_screen)
                .block(block),
            area,
        );
    }

    fn render_songs(&self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let border = Block::bordered();
        let items: Vec<ListItem> = self
            .tracks
            .iter()
            .map(|track| ListItem::new(track.label.clone()))
            .collect();
        //let sections = Layout::vertical([Constraint::Percentage(20), Constraint::Min(0)]);
        let list = List::new(items)
            .highlight_style(Modifier::REVERSED)
            .style(Color::Rgb(255, 255, 255))
            .block(Block::bordered().title("songs poorly ordered"));

        frame.render_stateful_widget(list, area, state);
    }

    fn render_image(&self, frame: &mut Frame, area: Rect, bytes: &Vec<u8>) -> Result<()> {
        let read_image = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        let decoded = read_image.decode()?;
        let picker = Picker::halfblocks();

        //let font_size = picker.font_size();
        //let size = Size::new(
        //    decoded.width().div_ceil(font_size.width as u32) as u16,
        //    decoded.height().div_ceil(font_size.height as u32) as u16,
        //);

        let size = Size::new(area.width, area.height);

        let image = picker.new_protocol(decoded, size, ratatui_image::Resize::Fit(None))?;
        let image = Image::new(&image);
        frame.render_widget(image, area);
        Ok(())
    }
}
