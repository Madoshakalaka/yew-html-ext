use yew_html_ext::html;

fn main() {
    let option = Some(42);
    
    // This should work but currently doesn't
    let _result = html! {
        match option {
            Some(n) => {
                <div>{ "Number: " }</div>
                <div>{ n }</div>
            }
            None => {
                <div>{ "No value" }</div>
            }
        }
    };
}