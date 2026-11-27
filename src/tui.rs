use crate::player::Queue;
use crate::track::Track;
use crate::{player, search};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use image::ImageReader;
use ratatui::layout::{Constraint, Layout, Size};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Chart, List, ListItem, ListState, Padding, Tabs, TitlePosition, Widget,
};
use ratatui::widgets::{GraphType, Paragraph};
use ratatui::{DefaultTerminal, Frame, symbols};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use ratatui_image::Image;
use ratatui_image::picker::Picker;
use std::io::Cursor;
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
    search_select: usize,
    search_result: Option<Vec<Track>>,
    random: bool,
    queue: Queue,
    queue_select: usize,
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
            search_select: 0,
            search_result: None,
            random: true,
            queue: Queue::new(),
            queue_select: 0,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        //println!("{:?}", &self.tracks);
        while !self.exit {
            self.queue.load(&mut self.player);
            let mut state = ListState::default().with_selected(Some(self.select));
            terminal.draw(|frame| self.render(frame, &mut state))?;

            self.handle_events()?;

            //println!("{:?}",self.player.handle.)
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, state: &mut ListState) {
        let v_layout = Layout::vertical([Constraint::Percentage(7), Constraint::Percentage(93)]);
        let h_layout: Layout =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(77)]);
        let [tabs, main] = frame.area().layout(&v_layout);
        let [left, right] = main.layout(&h_layout);

        let mut label = String::new();
        if self.random {
            label.push('r')
        }
        if self.loop_one {
            label.push('l')
        }

        self.render_tabs(frame, tabs, label);

        if self.active_screen == 1 {
            self.render_songs(frame, left, state);
        } else {
            self.render_home(frame, main);
        }
    }

    fn render_home(&mut self, frame: &mut Frame, area: Rect) {
        let v_layout = Layout::vertical([
            Constraint::Percentage(7),
            Constraint::Percentage(13),
            Constraint::Percentage(70),
            Constraint::Percentage(10),
        ]);

        let [search_area, list, main, playback_area] = area.layout(&v_layout);

        let h_layout = Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(77)]);

        let [playlists, visual] = main.layout(&h_layout);

        let visual_v_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [visualizer, other] = visual.layout(&visual_v_layout);

        let other_h_layout =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [idk, image_container] = other.layout(&other_h_layout);

        let title = if self.searching {
            "searching, esc to cancel"
        } else {
            "search, / to query"
        };
        let search = Paragraph::new(&*self.query).block(Block::bordered().title(title)); //Block::bordered().title("search");
        let playback = Block::bordered().title("playback");
        frame.render_widget(playback, playback_area);
        frame.render_widget(search, search_area);

        self.render_visualizer(frame, visualizer);

        self.render_playlists(frame, playlists);

        let mut state = ListState::default().with_selected(Some(self.search_select));
        self.render_query_list(frame, list, &mut state);

        if let Some(ref bytes) = self.player.image {
            self.render_image(frame, image_container, bytes).unwrap();
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                //KeyCode::Char(char) => self.exit(),
                KeyCode::Esc => {
                    if self.searching {
                        self.searching = false
                    } else {
                        self.exit()
                    }
                }
                KeyCode::Tab => self.active_screen = (self.active_screen + 1) % 2,
                KeyCode::Down => {
                    if self.active_screen == 1 {
                        if self.select == self.tracks.len() - 1 {
                            self.select = 0
                        } else {
                            self.select += 1
                        }
                    } else {
                        if let Some(ref results) = self.search_result {
                            if self.search_select == results.len() - 1 {
                                self.search_select = 0
                            } else {
                                self.search_select += 1
                            }
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
                    } else {
                        if let Some(ref results) = self.search_result {
                            if self.search_select == 0 {
                                self.search_select = results.len() - 1;
                            } else {
                                self.search_select -= 1
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    if self.active_screen == 1 {
                        //let _ = self.player.append(&mut self.tracks[self.select])?;
                        //self.player.play(&mut self.tracks[self.select])?
                        self.queue.append_from_track(&self.tracks[self.select]);
                    } else {
                        if !self.query.is_empty() && self.search_result.is_none() {
                            self.search()?;
                        } else {
                            if self.search_result.is_some() {
                                if let Some(ref tracks) = self.search_result {
                                    self.queue.append_from_track(&tracks[self.search_select]);
                                };
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if self.active_screen == 0 {
                        if self.searching {
                            self.query.pop();
                        }
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
                                '/' => {
                                    self.searching = true;
                                    self.query.clear();
                                    self.search_result = None
                                }
                                'r' => self.random = true,
                                'l' => self.loop_one = true,
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

    fn render_tabs(&self, frame: &mut Frame, area: Rect, label: String) {
        let block: Block = Block::bordered().title(label);
        frame.render_widget(
            Tabs::new(["Home", "List"])
                .select(self.active_screen)
                .block(block),
            area,
        );
    }

    fn render_songs(&self, frame: &mut Frame, area: Rect, state: &mut ListState) {
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

    fn render_query_list(&mut self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let Some(ref search_result) = self.search_result else {
            return;
        };
        let items: Vec<String> = search_result
            .iter()
            .map(|track| track.label.clone())
            .collect(); //ohhhh
        //so NOW it works
        //i LOVE collect
        let list = List::new(items)
            .block(Block::bordered())
            .highlight_style(Modifier::REVERSED);

        frame.render_stateful_widget(list, area, state);
    }

    fn render_image(&self, frame: &mut Frame, area: Rect, bytes: &Vec<u8>) -> Result<()> {
        let read_image = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        let decoded = read_image.decode()?;
        let picker = Picker::halfblocks();

        let size = Size::new(area.width, area.height);

        let image = picker.new_protocol(
            decoded,
            size,
            ratatui_image::Resize::Scale(Some(ratatui_image::FilterType::Lanczos3)),
        )?;
        let image = Image::new(&image);
        frame.render_widget(image, area);
        Ok(())
    }

    fn search(&mut self) -> Result<()> {
        if self.search_result.is_some() {
            self.search_result = None;
        };
        let input = self.query.to_owned();
        let tracks = self.tracks.to_owned();
        //ig clone works here?? i'm doing a lot of cloning in this function, but ik it's required
        //*somewhere* here, otherwise original list is gone. i assume tracks will not outlive this
        //function (wtf you mean, of course it wont. this isnt garbage collection, this is rust)
        self.search_result = Some(
            search::search(input, tracks)
                .iter()
                .map(|output| output.0.clone())
                .collect(),
        );
        self.searching = false;
        Ok(())
    }

    fn render_visualizer(&self, frame: &mut Frame, area: Rect) {
        let chart = Chart::default().block(Block::bordered().title("viz"));

        frame.render_widget(chart, area);
    }

    fn render_playlists(&mut self, frame: &mut Frame, area: Rect) {
        let list = List::new(["hi", "no"]).block(Block::bordered().title("playlists"));

        frame.render_widget(list, area);
    }
}
