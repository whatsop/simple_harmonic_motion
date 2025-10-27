mod physics;
use physics::{Object, Spring};

use nannou::prelude::*;

const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 1000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    spring_restlength: f32,
    spring_stiffness: f32,
    objects: Vec<Object>,

    main_window_id: WindowId,
}

fn model(app: &App) -> Model {
    let main_window_id = app
        .new_window()
        .view(view)
        .event(event)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable(false)
        .build()
        .unwrap();

    let object = Object::new(vec2(0.0, 0.0), vec2(0.0, 0.0), vec2(0.0, 0.0), 1.0);
    let objects = Vec::from([object]);

    let spring_stiffness = 4.0;
    let spring_restlength = 50.0;

    Model {
        spring_restlength,
        spring_stiffness,
        objects,
        main_window_id,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    if let KeyPressed(key) = event {
        if key == Key::Numpad1 && model.spring_stiffness >= 2.0 {
            model.spring_stiffness -= 1.0;
        }
        if key == Key::Numpad2 && model.spring_stiffness < 10.0 {
            model.spring_stiffness += 1.0;
        }
        if key == Key::Numpad4 && model.spring_restlength >= 20.0 {
            model.spring_restlength -= 10.0;
        }
        if key == Key::Numpad5 && model.spring_restlength <= 500.0 {
            model.spring_restlength += 10.0;
        }
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let mouse_position = app.mouse.position();

    for obj in &mut model.objects {
        let mut spring = Spring::new(
            model.spring_stiffness,
            model.spring_restlength,
            mouse_position,
        );

        spring.apply_force(obj);
        obj.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let mouse_position = app.mouse.position();
    draw.ellipse().xy(mouse_position).color(YELLOW).radius(5.0);
    for obj in &model.objects {
        obj.draw(&draw);
        draw.line()
            .start(mouse_position)
            .end(obj.position)
            .color(WHITE);
    }

    let window_rect = app.window(model.main_window_id).unwrap().rect();
    let x_offset = 250.0;
    let x_width = 450.0;
    draw.text(
        format!(
            "spring stiffness (numpad1 to decrease, numpad2 to increase): {}",
            model.spring_stiffness
        )
        .as_str(),
    )
    .x(window_rect.top_left().x + x_offset)
    .y(window_rect.top_left().y - 50.0)
    .width(x_width)
    .left_justify()
    .color(WHITE);
    draw.text(
        format!(
            "spring restlength (numpad4 to decrease, numpad5 to incease): {}",
            model.spring_restlength
        )
        .as_str(),
    )
    .x(window_rect.top_left().x + x_offset)
    .y(window_rect.top_left().y - 60.0)
    .width(x_width)
    .left_justify()
    .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
