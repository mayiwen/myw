use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    myw::{
        self,
        button::Button,
        message::Message,
        modal::Modal,
        table::{TabColumn, Table},
        tabset::{Tab, Tabset},
    },
    util::open_url,
};

use crate::models::title::Title;
use leptos::{prelude::*, reactive::spawn_local};
#[component]
pub fn Date() -> impl IntoView {
    let date_vie = RwSignal::new("".into());
    spawn_local(async move {
        let data = match crate::get_date().await {
                Ok(date) => date,
                Err(_) => String::new(),
        };
        date_vie.set(data);
    });

   
    
    view! {
        <myw::Gap/>
        <h3>api设置</h3>
        <h4>时间    </h4>
        <p>{date_vie} </p>
    
    }
}
