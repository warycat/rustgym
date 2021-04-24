use crate::message::Message;
use crate::model::Model;
use rustgym_consts::*;
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Message> {
    div![
        div![
            C!["searchbar"],
            div![
                C!["relative mt-3 mx-auto"],
                span![
                    C!["absolute inset-y-0 left-0 pl-3 flex items-center"],
                    svg![
                        C!["h-5 w-5 text-gray-500"],
                        attrs! {
                            At::ViewBox => "0 0 24 24",
                            At::Fill => "none"
                        },
                        path![attrs! {
                            At::D => "M21 21L15 15M17 10C17 13.866 13.866 17 10 17C6.13401 17 3 13.866 3 10C3 6.13401 6.13401 3 10 3C13.866 3 17 6.13401 17 10Z",
                            At::Stroke=>"currentColor",
                            At::StrokeWidth=>"2",
                            At::StrokeLinecap=>"round",
                            At::StrokeLineJoin=>"round"
                        }]
                    ]
                ],
                input![
                    C!["w-full border rounded-md pl-10 pr-4 py-2 focus:border-blue-500 focus:outline-none focus:shadow-outline"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => SEARCH_PLACEHOLDER,
                        At::Value => model.search_text,
                    },
                    input_ev(Ev::Input, Message::SearchTextChanged),
                    keyboard_ev("keydown", Message::KeyDown)
                ],
                div![
                    C!["autocom-box w-full border rounded-md", IF!(model.search_suggestions.is_empty() => "empty")],
                    model.search_suggestions.iter().map(|suggestion| {
                        let suggestion_clone = suggestion.clone();
                        div![
                            C!["pl-10 pr-4 py-2"],
                            suggestion_clone.to_string(),
                            ev(Ev::Click, |_| Message::QueryText(suggestion_clone))
                        ]
                    })
                ]
            ]
        ],
        table! [
            C!["table-auto", IF!(model.query_results.is_empty() => "empty")],
            thead![
                tr![
                    th!["id"],
                    th!["title"],
                    th!["from"],
                ]
            ],
            tbody![
                model.query_results.iter().map(|result|
                    tr![
                        td![result.id.to_string()],
                        td![
                            a![
                                attrs!{At::Href => result.href.to_string()},
                                result.title.to_string(),
                            ]
                        ],
                        td![result.from.to_string()]
                    ]
                )
            ],
        ],
        // model.all_clients.iter().filter(|client| client.streaming).map(|client|
        //     div![
        //         div![client.client_uuid.to_string()],
        //         video![
        //             C!["video"],
        //             source![
        //                 attrs!{
        //                     At::Src => format!("/stream/{}/playlist.m3u8", client.client_uuid.to_string()),
        //                     At::Type => "application/x-mpegURL"
        //                 }
        //             ],
        //             attrs!{
        //                 At::AutoPlay => true,
        //                 At::Controls => true,
        //             }
        //         ]
        //     ]
        // ),
    ]
}
