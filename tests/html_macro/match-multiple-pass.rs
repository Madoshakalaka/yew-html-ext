fn main() {
    _ = ::yew_html_ext::html! {
        match ::std::option::Option::Some(3u32) {
            ::std::option::Option::Some(_) => {
                <div>{"First"}</div>
                <div>{"Second"}</div>
            },
            ::std::option::Option::None => {},
        }
    }
}