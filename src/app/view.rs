use crate::logparse::parse_logfile;

use super::model::AppModel;
use super::{
    events::{AppInput, Outbox},
    model::LogFile,
};
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    prelude::*,
    AsyncComponentSender,
};
use relm4_components::{
    open_button::{OpenButton, OpenButtonSettings},
    open_dialog::OpenDialogSettings,
};

#[relm4::component(pub, async)]
impl AsyncComponent for AppModel {
    type Init = Option<LogFile>;

    type Input = AppInput;
    type Output = Outbox;
    type CommandOutput = ();

    view! {
        gtk::ApplicationWindow  {
            set_title: Some("JSON Log Viewer"),
            set_default_size: (1024,768),
            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_start: model.open_button.widget(),
            }
        }
    }

    // Initialize the UI.
    // Initialize the component.
    async fn init(
        log_file: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let open_button = OpenButton::builder()
            .launch(OpenButtonSettings {
                dialog_settings: OpenDialogSettings::default(),
                text: "Open file",
                recently_opened_files: Some(".recent_files"),
                max_recent_files: 10,
            })
            .forward(sender.input_sender(), AppInput::Open);
        let model = AppModel {
            log_file,
            open_button,
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            AppInput::Open(path) => {
                let parsed = parse_logfile(path).await;
                println!("Parsed: {parsed:?}")
            }
        }
    }
}
