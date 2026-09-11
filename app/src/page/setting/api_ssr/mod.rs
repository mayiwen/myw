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
    let time_num = RwSignal::new("".into());
    let fetch_date = move || {
        spawn_local(async move {
            let data = match crate::get_date().await {
                    Ok(date) => date,
                    Err(_) => String::new(),
            };
            date_vie.set(data);
        });
    };
    let fetch_time_num = move || {
        spawn_local(async move {
            let data = match crate::get_date_num().await {
                    Ok(date) => date,
                    Err(_) => String::new(),
            };
            time_num.set(data);
        });
    };

    fetch_date();
    fetch_time_num();

    let click_fn = move |_| fetch_date();
    let click_fn_get_time_num = move |_| fetch_time_num();
   
    
    view! {
        <myw::Gap/>
        <h3>api设置</h3>
        <h4>时间    </h4>
         <Button  on_click=click_fn >获取时间</Button>
        <p>{date_vie} </p>
         <Button  on_click=click_fn_get_time_num >获取时间戳</Button>
        <p>{time_num} </p>
    
    }
}
