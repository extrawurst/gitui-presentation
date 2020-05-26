// tui-rs

// draw `txt` iterator in centered box
let area = ui::centered_rect(30, 20, f.size());
f.render_widget(
    Paragraph::new(txt)
        .block(
            Block::default()
                .title("title")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Left)
        .wrap(true),
    area,
);