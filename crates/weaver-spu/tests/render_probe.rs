//! Temporary measurement: what does `llama_chat_apply_template` actually emit?
#![cfg(feature = "gguf")]
use std::ffi::CString;
use weaver_spu::artifact;

fn render(template: &str) -> Result<String, i32> {
    let tmpl = CString::new(template).map_err(|_| -99)?;
    let mk = |r: &str, c: &str| (CString::new(r).unwrap(), CString::new(c).unwrap());
    let held = vec![mk("system", "S"), mk("user", "U"), mk("assistant", "A")];
    let chat: Vec<llama_cpp_sys_2::llama_chat_message> = held
        .iter()
        .map(|(r, c)| llama_cpp_sys_2::llama_chat_message {
            role: r.as_ptr(),
            content: c.as_ptr(),
        })
        .collect();
    let mut buf = vec![0u8; 16384];
    let n = unsafe {
        llama_cpp_sys_2::llama_chat_apply_template(
            tmpl.as_ptr(),
            chat.as_ptr(),
            chat.len(),
            true,
            buf.as_mut_ptr().cast::<std::os::raw::c_char>(),
            buf.len() as i32,
        )
    };
    if n < 0 {
        return Err(n);
    }
    buf.truncate(n as usize);
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

#[test]
fn what_the_render_emits() {
    for path in [
        "/tmp/phi4mini.gguf",
        "/opt/weaver/models/smollm2-360m-instruct-q8_0.gguf",
        "/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf",
        "/opt/weaver/models/gemma4-31b-it-Q8_0.gguf",
        "/opt/weaver/models/nemotron-cascade-Q8_0.gguf",
    ] {
        let Ok(h) = artifact::pin(std::path::Path::new(path))
            .and_then(|mut p| artifact::read_header(&mut p))
        else {
            println!("\n{path}\n  header unreadable");
            continue;
        };
        let Some(t) = h.chat_template.as_deref() else {
            println!("\n{path}\n  arch={} NO TEMPLATE", h.family.0);
            continue;
        };
        println!("\n{path}\n  arch={}", h.family.0);
        println!("  source has <|user|>: {}", t.contains("<|user|>"));
        match render(t) {
            Ok(out) => println!("  RENDER: {:?}", &out[..out.len().min(220)]),
            Err(e) => println!("  RENDER FAILED rc={e}"),
        }
    }
}
