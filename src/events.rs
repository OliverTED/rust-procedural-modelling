use glium;

use glium::glutin::Event::*;
use glium::glutin::MouseScrollDelta;

use parser;
use modeler;

use visualizer::*;

use visualizer;

struct State {
    viewer:Viewer,
    focused:bool,
    running:bool,
    geometry: LoadedGeometry,
}

fn reload_geometry(display:&Display) -> LoadedGeometry {
    let grammar = parser::parse("default.txt");
    let model = modeler::build(&grammar);
    let geometry = GeometryData::from_derivation(&model);

    load_geometry(&display, &geometry)
}


fn handle_events(display:&Display, state:&mut State) {
    let window = display.get_window().unwrap();
    
    let (middle_x, middle_y) =
        if let Some((x,y)) = window.get_inner_size_points() {
            (x as i32/2, y as i32/2)
        } else {
            (100i32, 100i32)
        };

    let viewer = &mut state.viewer;
    
    for ev in display.poll_events() {
        match ev {
            glium::glutin::Event::Closed => state.running=false,
            Focused(nstate) => state.focused = nstate,
            _ => {}
        };

        if state.focused {
            match ev {
                MouseMoved((x, y)) => {
                    if (x,y) != (middle_x, middle_y) {
                        window.set_cursor_position(middle_x, middle_y).unwrap();
                        viewer.mouse_move(x,y);
                    } else {
                        viewer.set_last(x, y);
                    }
                },
                MouseWheel(MouseScrollDelta::LineDelta(x, y)) => viewer.mouse_scroll(x, y),
                MouseWheel(MouseScrollDelta::PixelDelta(x, y)) => viewer.mouse_scroll(x, y),
                MouseInput(state, button) => viewer.mouse_button(state, button),

                ReceivedCharacter(ch) => {
                    match ch {
                        'r' => state.geometry = reload_geometry(&display),
                        'q' => state.running = false,
                        _ => {},
                    }
                },
                
                _ => {}
            };
        }
    }
}


pub fn run_main() {
    let display = visualizer::create_display();

    let mut state = State {
        viewer: visualizer::Viewer::new(),
        focused: true,
        running: true,
        geometry: reload_geometry(&display),
    };

    while state.running {
        visualizer::draw_frame(&display, &state.viewer, Some(&state.geometry));

        handle_events(&display, &mut state);
    }
}
