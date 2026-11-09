use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 宣告可變變數 score（預設 50 分）
    let mut score = use_signal(|| 50);

    rsx! {
        // 修正為透過 asset!巨集正確打包外部 CSS
        document::Link { rel: "stylesheet", href: asset!("assets/style.css") }

        div { class: "container",
            h1 { "🎓 高中生 Rust 互動學習專案" }
            p { class: "score-text", "目前成績：{score} 分" }

            // 加分按鈕
            button {
                onclick: move |_| {
                    score.set((score() + 10).min(100)); // 最多100分
                },
                class: "btn-add",
                "加 10 分"
            }

            // 重設按鈕
            button {
                onclick: move |_| {
                    score.set(50);
                },
                class: "btn-reset",
                "重設分數"
            }

            // if 條件判斷：即時顯示及格或不及格
            div { class: "result-box",
                if score() >= 60 {
                    p { class: "pass", "🎉 太棒了！成績及格（>= 60分）！" }
                } else {
                    p { class: "fail", "⚠️ 尚未及格（未滿 60分），請繼續加油！" }
                }
            }
        }
    }
}
