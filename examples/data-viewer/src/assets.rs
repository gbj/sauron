use sauron::{
    html::{
        attributes::*,
        units::*,
    },
    svg::{
        attributes::*,
        *,
    },
    Node,
};

pub fn svg_table_icon<MSG>() -> Node<MSG>
where
    MSG: Clone,
{
    svg(vec![ xmlns("http://www.w3.org/2000/svg"), width(20), height(20), viewBox([0, 0, 24, 24])],
            vec![ path(vec![ d("M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"), fill("rgba(128,128,128,1)") ],
                vec![])
            ])
}

pub fn svg_search_icon<MSG>(w: i32, h: i32, kolor: &'static str) -> Node<MSG>
where
    MSG: Clone,
{
    svg( vec![version("1.1"), width(px(w)), height(px(h)), viewBox([0, 0, 512, 512]), enable_background("new 0 0 512 512")],
    vec![path(vec![d("M445,386.7l-84.8-85.9c13.8-24.1,21-50.9,21-77.9c0-87.6-71.2-158.9-158.6-158.9C135.2,64,64,135.3,64,222.9c0,87.6,71.2,158.9,158.6,158.9c27.9,0,55.5-7.7,80.1-22.4l84.4,85.6c1.9,1.9,4.6,3.1,7.3,3.1c2.7,0,5.4-1.1,7.3-3.1l43.3-43.8C449,397.1,449,390.7,445,386.7zM222.6,125.9c53.4,0,96.8,43.5,96.8,97c0,53.5-43.4,97-96.8,97c-53.4,0-96.8-43.5-96.8-97C125.8,169.4,169.2,125.9,222.6,125.9z"),
             fill(kolor)], vec![])])
}

pub fn sort_btn_asc<MSG>(w: i32, h: i32, kolor: &'static str) -> Node<MSG>
where
    MSG: Clone,
{
    svg(vec![ version("1.1"), width(px(w)), height(px(h)), viewBox([0, 0, 512, 512]), enable_background("new 0 0 512 512")],
    vec![polygon(vec![points("256.5,64.5 64.5,256.5 176.5,256.5 176.5,448.5 336.5,448.5 336.5,256.5 448.5,256.5"), fill(kolor)], vec![]) ])
}

pub fn close_button<MSG>(w: i32, h: i32, kolor: &'static str) -> Node<MSG>
where
    MSG: Clone,
{
    svg(vec![version("1.1"), width(px(w)), height(px(h)), viewBox([0, 0, 512, 512]), enable_background("new 0 0 512 512")],
        vec![path(vec![d("M256,33C132.3,33,32,133.3,32,257c0,123.7,100.3,224,224,224c123.7,0,224-100.3,224-224C480,133.3,379.7,33,256,33zM364.3,332.5c1.5,1.5,2.3,3.5,2.3,5.6c0,2.1-0.8,4.2-2.3,5.6l-21.6,21.7c-1.6,1.6-3.6,2.3-5.6,2.3c-2,0-4.1-0.8-5.6-2.3L256,289.8l-75.4,75.7c-1.5,1.6-3.6,2.3-5.6,2.3c-2,0-4.1-0.8-5.6-2.3l-21.6-21.7c-1.5-1.5-2.3-3.5-2.3-5.6c0-2.1,0.8-4.2,2.3-5.6l75.7-76l-75.9-75c-3.1-3.1-3.1-8.2,0-11.3l21.6-21.7c1.5-1.5,3.5-2.3,5.6-2.3c2.1,0,4.1,0.8,5.6,2.3l75.7,74.7l75.7-74.7c1.5-1.5,3.5-2.3,5.6-2.3c2.1,0,4.1,0.8,5.6,2.3l21.6,21.7c3.1,3.1,3.1,8.2,0,11.3l-75.9,75L364.3,332.5z"),
        fill(kolor)], vec![])
        ]
    )
}
