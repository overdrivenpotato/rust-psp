#![no_std]
#![no_main]

use core::iter::FromIterator;

use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{
    layout::Margin,
    style::{Color, Stylize},
    text::{Line, Masked, Span, Text},
    widgets::{Block, BorderType, Paragraph, Wrap},
    Terminal,
};

use psp::embedded_graphics::Framebuffer;

psp::module!("ratatui_example", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let mut disp = Framebuffer::new();

    let backend = EmbeddedBackend::new(&mut disp, EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|frame| {
                let area = frame.area();
                let short_line = "Slice, layer, and bake the vegetables. ";
                let long_line = short_line.repeat((area.width as usize) / short_line.len() + 2);
                let lines = Text::from_iter([
                    "Recipe: Ratatouille".into(),
                    "Ingredients:".bold().into(),
                    Line::from_iter([
                        "Bell Peppers".into(),
                        ", Eggplant".italic(),
                        ", Tomatoes".bold(),
                        ", Onion".into(),
                    ]),
                    Line::from_iter([
                        "Secret Ingredient: ".underlined(),
                        Span::styled(Masked::new("herbs de Provence", '*'), Color::Red),
                    ]),
                    "Instructions:".bold().yellow().into(),
                    long_line.green().italic().into(),
                ]);

                let paragraph = Paragraph::new(lines)
                    .style(Color::White)
                    .scroll((0, 0))
                    .wrap(Wrap { trim: true })
                    .block(Block::bordered().border_type(BorderType::Rounded));

                frame.render_widget(
                    paragraph,
                    area.inner(Margin {
                        horizontal: 5,
                        vertical: 5,
                    }),
                );
            })
            .unwrap();
    }
}
