use yew::prelude::*;
use yew_router::prelude::*;

use crate::organism::common::{Footer, Header};
use crate::routes::Route;

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <Redirect<Route> to={Route::BookList} />
    }
}
