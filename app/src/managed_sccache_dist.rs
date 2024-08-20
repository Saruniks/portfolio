use fallout_ui::components::divider::Divider;
use fallout_ui::{
    components::{
        buttons::primary_link_button::PrimaryLinkButton,
        typography::{body_text::BodyText, header::Header},
    },
    utils::window,
};
use yew::prelude::*;

#[function_component]
pub fn ManagedSccacheDistPage() -> Html {
    html! {
        <>
            <Header class="text-4xl mb-4">{"Managed Distributed sccache"}</Header>

            <Header class="mb-1">{"About sccache and sccache-dist"}</Header>
            <Divider class="mb-4"/>

            <BodyText> {"sccache is a open-source ccache-like compiler caching tool created in mozilla. It is used as a compiler wrapper and avoids compilation when possible, storing cached results either on local disk or in one of several cloud storage backends."} </BodyText>
            <br/>
            <BodyText> {"sccache includes support for caching the compilation of C/C++ code, Rust, as well as NVIDIA's CUDA using nvcc, and clang."} </BodyText>
            <br/>
            <BodyText> {"sccache also provides icecream-style distributed compilation (automatic packaging of local toolchains) for all supported compilers (including Rust)."}</BodyText>
            <br/>

            <Header class="mb-1">{"sccache usage"}</Header>
            <Divider class="mb-4"/>

            <BodyText> {"One way to install sccache is via cargo:"} </BodyText>
            <pre>
                {"cargo install sccache --locked"}
            </pre>

            <BodyText> {"Environment variable "} <code>{"RUSTC_WRAPPER"}</code> {" can be used to run sccache when compiling Rust code:"} </BodyText>
            <pre> {
r###"export RUSTC_WRAPPER=/path/to/sccache
cargo build"### } </pre>
            <br/>

            <Header class="mb-1">{"sccache-dist usage"}</Header>
            <Divider class="mb-4"/>

            <BodyText> {"Either install pre-built sccache binaries, or build sccache locally with the dist-client and dist-server features enabled:"} </BodyText>

            <pre> {"cargo build --release --features=\"dist-client dist-server\""} </pre>

            <BodyText> {"Start the scheduler by running:"} </BodyText>

            <pre> {"sccache-dist scheduler --config scheduler.conf"} </pre>

            <BodyText> {"Start the server by running:"} </BodyText>

            <pre> {"sudo sccache-dist server --config server.conf"} </pre>

            <BodyText> {"Configure a client by creating client config file in "} <code> {"~/.config/sccache/config"} </code> {" (on Linux). A minimal example looks like:"} </BodyText>
            <pre> {
r###"[dist]
# The URL used to connect to the scheduler (should use https, given an ideal
# setup of a HTTPS server in front of the scheduler)
scheduler_url = "https://192.168.1.1"
# Used for mapping local toolchains to remote cross-compile toolchains. Empty in
# this example where the client and build server are both Linux.
toolchains = []
# Size of the local toolchain cache, in bytes (5GB here, 10GB if unspecified).
toolchain_cache_size = 5368709120

            
[dist.auth]
type = "token"
# This should match the `client_auth` section of the scheduler config.
token = "my client token"
"###        } </pre>

            <Header class="mb-1">{"So what am I working on?"}</Header>
            <Divider class="mb-4"/>

            <BodyText> {"I'm currently developing managed distributed sccache, where users could deploy sccache-dist instances with a couple of clicks. After deploying the instance they would get the scheduler_url and client token to be used for distributed builds:"} </BodyText>
            <div class="flex justify-center items-center">
                <div class="max-w-3xl">
                    <img src="/images/panel.png" alt="panel" />
                </div>
            </div>

            <br/>
            <BodyText> {"Logs could be accessed in a centralized place:"} </BodyText>
            <br/>

            <div class="flex justify-center items-center mb-4">
                <div class="max-w-5xl">
                    <img src="/images/logs.png" alt="logs" />
                </div>
            </div>

            <BodyText> {"See managed distributed sccache in action: "} <PrimaryLinkButton onclick={move |_| {
                        window()
                            .unwrap()
                            .open_with_url("https://vendenic.link")
                            .unwrap();
                        }
                    }>{"https://vendenic.link ↗️"}
                </PrimaryLinkButton>
            </BodyText>



        </>
    }
}
