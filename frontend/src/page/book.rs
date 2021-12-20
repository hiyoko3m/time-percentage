use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::atom::common::SectionTitle;
use crate::organism::book::{Book, BooksList};
use crate::organism::common::{DeleteModal, Footer, Header};
use crate::routes::Route;

#[derive(Deserialize)]
struct BooksResponse {
    books: Vec<Book>,
}

#[function_component(BooksListPage)]
pub fn list_books() -> Html {
    let history = use_history().expect("history API encounters a critical error");

    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with_deps(
            move |_| {
                let books = books.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response: BooksResponse = Request::get("http://localhost:8000/v1/books")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    books.set(response.books);
                });
                || ()
            },
            (),
        );
    }

    let on_click = {
        let history = history.clone();
        Callback::once(move |book: Book| history.push(Route::BookDetail { id: book.id }))
    };
    let on_edit_click = {
        let history = history.clone();
        Callback::once(move |book: Book| history.push(Route::BookEdit { id: book.id }))
    };

    let delete_target = use_state(|| Book::default());

    // アイテム一覧内の削除ボタンが押されたときのコールバック
    // まだ確認モーダルを出すだけで、実際の削除はしない
    let on_delete_click = {
        let delete_target = delete_target.clone();
        Callback::from(move |book: Book| delete_target.set(book))
    };

    let delete_modal_id = "deleteModal".to_string();
    let delete_modal_message = format!("{} を本当に削除しますか？", (*delete_target).title);

    let on_delete_confirmed = {
        let delete_target = delete_target.clone();
        Callback::from(move |_| {
            let delete_target = delete_target.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::delete(
                    format!("http://localhost:8000/v1/books/{}", (*delete_target).id).as_str(),
                )
                .send()
                .await
                .unwrap();
            });
        })
    };

    html! {
        <>
            <Header />
            <div class="container-lg">
                <SectionTitle title={"登録された本の一覧".to_string()} />
                <BooksList books={(*books).clone()} {on_click} {on_edit_click} {on_delete_click} delete_target={format!("#{}", delete_modal_id)}/>
            </div>
            <DeleteModal title={"確認".to_string()} message={delete_modal_message.clone()} id={delete_modal_id.clone()} label={"deleteModalLabel".to_string()} on_click={on_delete_confirmed} />
            <Footer />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct BooksDetailProps {
    pub id: usize,
}

#[function_component(BooksDetail)]
pub fn describe_book(props: &BooksDetailProps) -> Html {
    html! {
        <>
            <div>{
                format!("describe book {}", props.id.clone())
            }</div>
        </>
    }
}