use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, UnitPoint, WindowDesc};

use print_ui::editor::EditView;

use crate::delegate::Delegate;
use std::path::{Path, PathBuf};
use std::sync::Arc;
pub use support::line;

pub mod command;
pub mod delegate;
pub mod menu;
pub mod print_ui;
pub mod support;
pub mod theme;

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

#[derive(Clone, Data, Lens)]
struct AppState {
    title: String,
    workspace: Workspace,
    params: Params,
}

#[derive(Clone, Data, Lens)]
struct Workspace {
    pub current_file: Option<Arc<Path>>,
    pub current_dir: Option<Arc<Path>>,
    pub input_text: String,
}

impl Workspace {
    pub fn set_file(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        self.current_file = path;
    }

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        self.current_dir = path;
    }
}

#[derive(Clone, Data, Lens)]
struct Params {
    debug_layout: bool,
}

fn navigation_bar() -> impl Widget<AppState> {
    let label = Label::new(|data: &Workspace, _: &Env| match &data.current_dir {
        None => {
            format!("")
        }
        Some(path) => {
            format!("{}", path.to_owned().display())
        }
    });

    Flex::row()
        .with_child(label.with_text_color(Color::BLACK))
        .padding(10.0)
        .expand_width()
        .lens(AppState::workspace)
        .background(line::hline())
        .align_horizontal(UnitPoint::LEFT)
}

fn status_bar() -> impl Widget<AppState> {
    Flex::row()
        .with_default_spacer()
        .with_flex_child(Label::new("status bar").with_text_color(Color::BLACK), 1.0)
        .with_default_spacer()
        .with_flex_child(Label::new("time").with_text_color(Color::BLACK), 1.0)
        .lens(AppState::params)
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(navigation_bar())
        .with_flex_child(EditView::new().center(), 1.0)
        .with_child(status_bar())
        .background(LIGHTER_GREY)
}

pub fn main() {
    let title = "Print UI";

    let menu = menu::menus();

    let main_window = WindowDesc::new(crate::theme::wrap_in_theme_loader(make_ui()))
        .window_size((720., 600.))
        .with_min_size((620., 300.))
        .menu(menu)
        .title(title);

    let workspace = Workspace {
        current_file: None,
        current_dir: None,
        input_text: "".into(),
    };

    let params = Params {
        debug_layout: false,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(AppState {
            title: title.to_string(),
            workspace,
            params,
        })
        .expect("Failed to launch application");
}
