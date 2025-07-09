#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

use ::yew_html_ext::html;

fn compile_pass() {
    // Test textarea with defaultvalue
    _ = html! {
        <textarea defaultvalue="Default content" />
    };

    // Test textarea with dynamic defaultvalue
    let default_text = "Dynamic default";
    _ = html! {
        <textarea defaultvalue={default_text} />
    };

    // Test textarea with both value and defaultvalue
    _ = html! {
        <textarea value="Current value" defaultvalue="Default value" />
    };

    // Test textarea with optional defaultvalue
    let optional_default: ::std::option::Option<&::std::primitive::str> = ::std::option::Option::Some("Optional default");
    _ = html! {
        <textarea defaultvalue={optional_default} />
    };

    // Test textarea with None defaultvalue
    let none_default: ::std::option::Option<&::std::primitive::str> = ::std::option::Option::None;
    _ = html! {
        <textarea defaultvalue={none_default} />
    };

    // Test dynamic textarea tag with defaultvalue
    let tag = "textarea";
    _ = html! {
        <@{tag} defaultvalue="Dynamic textarea default" />
    };

    // Test textarea with all attributes
    _ = html! {
        <textarea 
            value="Current" 
            defaultvalue="Default" 
            rows="10" 
            cols="30" 
            placeholder="Enter text"
            readonly=true
        />
    };

    // Test that defaultvalue on input element is treated as regular attribute
    _ = html! {
        <input type="text" defaultvalue="This should be a regular attribute" />
    };

    // Test that defaultvalue on div is treated as regular attribute
    _ = html! {
        <div defaultvalue="This should be a regular attribute">
            {"Content"}
        </div>
    };

    // Test nested textareas
    _ = html! {
        <form>
            <textarea defaultvalue="First textarea" />
            <textarea value="Second textarea" defaultvalue="Second default" />
            <textarea />
        </form>
    };
}

fn main() {}