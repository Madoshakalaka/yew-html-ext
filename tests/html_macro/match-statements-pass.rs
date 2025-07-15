use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    value: i32,
}

#[function_component]
fn Component(props: &Props) -> Html {
    ::yew_html_ext::html! {
        <div>{props.value}</div>
    }
}

fn compile_pass() {
    // Test 1: Basic match with block containing statements
    _ = ::yew_html_ext::html! {
        match true {
            true => {
                let a = 1;
                let b = 2;
                <div>{a + b}</div>
            },
            false => <div>{"false"}</div>,
        }
    };
    
    // Test 2: Match with multiple elements in block
    _ = ::yew_html_ext::html! {
        match 5 {
            1..=3 => {
                let msg = "low";
                <div>{msg}</div>
                <span>{"Range: 1-3"}</span>
            },
            4..=6 => {
                <h1>{"Medium"}</h1>
                <p>{"Value is between 4 and 6"}</p>
            },
            _ => <div>{"High"}</div>,
        }
    };
    
    // Test 3: Nested match with blocks
    _ = ::yew_html_ext::html! {
        match true {
            true => {
                let prefix = "Outer is true, ";
                match 1 {
                    1 => <div>{format!("{}{}", prefix, "x is 1")}</div>,
                    _ => {
                        let suffix = "x is not 1";
                        <div>{format!("{}{}", prefix, suffix)}</div>
                    }
                }
            },
            false => <div>{"Outer is false"}</div>,
        }
    };
    
    // Test 4: Match with guards and blocks
    let x = 10;
    _ = ::yew_html_ext::html! {
        match x {
            n if n < 5 => {
                let msg = "Less than 5";
                <div>{msg}</div>
            },
            n if n >= 5 => {
                let msg = "Greater or equal to 5";
                <div>{msg}</div>
                <span>{n}</span>
            },
            _ => <div>{"Unreachable"}</div>,
        }
    };
    
    // Test 5: Match with single element in block (optimization test)
    _ = ::yew_html_ext::html! {
        match true {
            true => {
                <div>{"Single element in block"}</div>
            },
            false => <div>{"false"}</div>,
        }
    };
    
    // Test 6: Match with Option pattern and blocks
    let opt = Some(42);
    _ = ::yew_html_ext::html! {
        match opt {
            Some(n) => {
                let doubled = n * 2;
                <div>{"Value: "}{n}</div>
                <div>{"Doubled: "}{doubled}</div>
            },
            None => <div>{"No value"}</div>,
        }
    };
    
    // Test 7: Match with components in blocks
    _ = ::yew_html_ext::html! {
        match true {
            true => {
                let value = 42;
                <Component value={value} />
            },
            false => {
                <Component value={0} />
            },
        }
    };
    
    // Test 8: Match with Result pattern
    let result: Result<i32, &str> = Ok(42);
    _ = ::yew_html_ext::html! {
        match result {
            Ok(value) => {
                let msg = format!("Success: {}", value);
                <div class="success">{msg}</div>
            },
            Err(e) => <div class="error">{e}</div>,
        }
    };
}

fn main() {}