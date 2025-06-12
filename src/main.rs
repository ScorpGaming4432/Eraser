use pulldown_cmark::{Parser, Options, Event, Tag, HeadingLevel};
use eframe::egui::{self, RichText, ScrollArea, Ui};

#[derive(Debug)]
enum StyleMarker {
    Italic,
    Bold,
    Heading(HeadingLevel),
    Codeblock
}

pub fn render_markdown_preview(ui: &mut Ui, markdown_input: &str) {
    let parser = Parser::new_ext(markdown_input, Options::ENABLE_STRIKETHROUGH);

    let mut style_stack: Vec<StyleMarker> = Vec::new();
    let mut bullet_indent = false;

    ScrollArea::vertical().show(ui, |ui| {
        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Emphasis => style_stack.push(StyleMarker::Italic),
                    Tag::Strong => style_stack.push(StyleMarker::Bold),
                    Tag::Heading(level, _, _) => style_stack.push(StyleMarker::Heading(level)),
                    Tag::Item => bullet_indent = true,
                    Tag::CodeBlock(_) => style_stack.push(StyleMarker::Codeblock),
                    _ => {}
                },

                Event::End(tag) => match tag {
                    Tag::Emphasis | Tag::Strong | Tag::Heading(..) | Tag::CodeBlock(_) => {
                        style_stack.pop();
                    }
                    Tag::Item => bullet_indent = false,
                    _ => {}
                },

                Event::Text(text) => {
                    // THIS is your `styled`:
                    let mut styled = RichText::new(text.to_string());

                    for style in &style_stack {
                        match style {
                            StyleMarker::Italic => styled = styled.italics(),
                            StyleMarker::Bold => styled = styled.strong(),
                            StyleMarker::Heading(level) => {
                                styled = match level {
                                    HeadingLevel::H1 => styled.size(24.0).strong(),
                                    HeadingLevel::H2 => styled.size(20.0).strong(),
                                    HeadingLevel::H3 => styled.size(18.0).strong(),
                                    HeadingLevel::H4 => styled.size(16.0),
                                    HeadingLevel::H5 => styled.size(14.0),
                                    HeadingLevel::H6 => styled.size(12.0),
                                };
                            }
                            StyleMarker::Codeblock => styled = styled.monospace(),
                        }
                    }

                    if bullet_indent {
                        styled = RichText::new(format!("• {}", styled.text()));
                    }

                    ui.label(styled);
                }

                Event::SoftBreak | Event::HardBreak => {
                    ui.label(" ");
                }

                Event::Rule => {
                    ui.separator();
                }

                _ => {
                    // skip html, code inline, footnotes, etc for now
                }
            }
        }
    });
}


fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions::default();
  eframe::run_native(
    "Markdown Editor",
    options,
    Box::new(|_cc| Ok(Box::new(Eraser::default()))),
  )
}

pub struct Eraser {
    pub editor_contents: String,
    pub last_file_path: Option<std::path::PathBuf>,
    pub is_fullscreen: bool,
    pub markdown_input: String,
    pub is_dirty: bool,

    // cokolwiek
}
impl Default for Eraser {
    fn default() -> Self {
        Self {
            editor_contents: String::new(),
            last_file_path: None,
            is_fullscreen: false,
            markdown_input: String::new(),
            is_dirty: true,
        }
    }
}



impl eframe::App for Eraser {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          ui.label("TODO: system plików");
          if ui.button("New").clicked() {
            self.markdown_input.clear();
            ui.close_menu();
          }
          if ui.button("Open...").clicked() {
            let file = rfd::FileDialog::new()
              .add_filter("Markdown", &["md", "markdown", "txt"])
              .set_directory(".")
              .pick_file();

            if let Some(path) = file {
              match std::fs::read_to_string(&path) {
                Ok(content) => self.editor_contents = content,
                Err(err) => eprintln!("Failed to load file: {}", err),
              }
            }

            ui.close_menu();
          }
          if ui.button("Save").clicked() {
            let file = rfd::FileDialog::new()
              .add_filter("Markdown", &["md", "markdown", "txt"])
              .set_file_name("Untitled.md")
              .save_file();

            if let Some(path) = file {
              if let Err(err) = std::fs::write(&path, &self.editor_contents) {
                eprintln!("Failed to save file: {}", err);
              }
            }

            ui.close_menu();
          }
          if ui.button("Save as...").clicked() {
            let file = rfd::FileDialog::new()
              .add_filter("Markdown", &["md", "markdown", "txt"])
              .set_file_name("Untitled.md")
              .save_file();

            if let Some(path) = file {
              if let Err(err) = std::fs::write(&path, &self.editor_contents) {
                eprintln!("Failed to save file: {}", err);
              }
            }

            ui.close_menu();
          }
          if ui.button("Import...").clicked() {
            // future: save logic
            ui.close_menu();
          }
          if ui.button("Export...").clicked() {
            // future: save logic
            ui.close_menu();
          }
          if ui.button("Exit").clicked() {
            // ¯\_(ツ)_/¯
            ui.close_menu();
            std::process::exit(0);
          }
        });

        ui.menu_button("Edit", |ui| {
          ui.label("TODO: Ctrl+Z, Ctrl+Y/Ctrl+Shift+Z");
          if ui.button("Undo").clicked() {
            ui.close_menu();
          }
          if ui.button("Redo").clicked() {
            ui.close_menu();
          }
        });

        ui.menu_button("View", |ui| {
          ui.label("TODO: cokolwiek tu jest");
          if ui.button("pasek narzędzi").clicked() {
            // future: save logic
            ui.close_menu();
          }
          if ui.checkbox(&mut self.is_fullscreen, "Fullscreen").changed() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.is_fullscreen));
          }
          if ui.button("ustawienia lokalne i globalne").clicked() {
            // future: save logic
            ui.close_menu();
          }
          if ui.button("licznik slow i znakow").clicked() {
            // future: save logic
            ui.close_menu();
          }
          //if ui.checkbox()
        });

        ui.menu_button("Help", |ui| {
          ui.label("TODO: formularz kontaktowy/ git issues");
          if ui.button("Github").clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab("https://github.com/ScorpGaming4432/eraser"));
            ui.close_menu();
          }
          if ui.button("Website").clicked() {
            ui.ctx().open_url(egui::OpenUrl::new_tab("https://scorpgaming4432.github.io/"));
            ui.close_menu();
          }
        });
      });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.columns(2, |columns| {
        // Left panel: Markdown input
        columns[0].vertical(|ui| {
          ui.heading("Markdown Editor");
          ui.add(egui::TextEdit::multiline(&mut self.markdown_input)
            .font(egui::TextStyle::Monospace)
            .desired_rows(30)
            .desired_width(f32::INFINITY),
          );
        });

        // Right panel: Markdown preview
        columns[1].vertical(|ui| {
          ui.heading("Preview");
          render_markdown_preview(ui, &self.markdown_input);
        });

      });
    });

  }
}
