mod at_about;
mod at_post;
mod json;

use ibex::prelude::*;
use std::collections::HashSet;

use crate::posts::{Post, PostList};
use crate::views::{icons, list_item, post_special_icon, sentence_case, use_base};

pub use at_about::*;
pub use at_post::*;
pub use json::*;

pub fn at_index(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base [
        "",
        None,
        posts,
    ] {
        ol ."post-list" [
            reversed!,
            start=posts.first().index,
        ] {
            [:for post in posts.into_iter().rev() {
                @list_item [&post]
            }]
        }
    }}
}

pub fn at_favorites(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base [
        "Plej bonaj",
        None,
        posts,
    ] {
        h2 { "Plej bonaj bildstrioj" }

        h3 { "(Laŭ mia opinio)" }
        ol ."post-list" [
            reversed!,
            start=posts.first().index,
        ] {
            [:for post in posts.into_iter().rev() {
                [:if post.get().props.good {
                    @list_item [&post]
                }]
            }]
        }
    }}
}

pub fn at_404(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base [
        "404",
        None,
        &posts,
    ] {
        h3 { "Paĝo ne trovita!" }
        p {
            "404 - Not found"
        }
    }}
}

fn posts_percent<F>(posts: &PostList, predicate: F) -> usize
where
    F: Fn(&Post) -> bool,
{
    posts.iter().filter(|post| predicate(post.get())).count() * 100 / posts.len()
}

fn posts_names(posts: &PostList) -> [Vec<(String, bool)>; 2] {
    let mut seen = HashSet::new();
    let speakers = posts
        .iter()
        .filter_map(|post| post.get().transcript.clone())
        .flat_map(|transcript| transcript.names())
        .filter(|name| seen.insert(name.clone()));

    let (mut common, mut uncommon): (Vec<_>, Vec<_>) =
        speakers.partition(|(_, uncommon)| !uncommon);
    common.sort();
    uncommon.sort();
    [common, uncommon]
}

pub fn at_list(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base [
        "Alia listo",
        None,
        posts,
    ] {
        br/
        div ."big-list" {
            div ."stats" {
                table {
                    [:where let percent_new = posts_percent(posts, |post| !post.is_old); {
                        tr { td/        td { b { [percent_new]         "%" } } td { "Novaj" } }
                    }]
                }
            }
            div ."names" {
                [:where
                    let [names_common, names_uncommon] = posts_names(posts);
                    fn section(title: &str, names: Vec<(String, bool)>) -> View {
                        view!{
                            div {
                                h3 { [title] }
                                p {
                                    [:for (i, (name, _)) in names.into_iter().enumerate() {
                                        [:if i > 0 { ","~ }]
                                        i { [sentence_case(&name, true)] }
                                    }]
                                }
                            }
                        }
                    }
                {
                    @section["Oftaj Nomoj",    names_common]
                    @section["Maloftaj Nomoj", names_uncommon]
                }]
            }
            div ."legend" {
                table {
                    [:where let percent_good = posts_percent(posts, |post| post.props.good); {
                        tr { td { [icons::GOOD] }   td { "Bona" ~ i { "(" [percent_good] "%)" } } } }]
                    tr { td { [icons::TRANSCRIPT] } td { "Transskribita" } }
                    tr { td { [icons::OLD] }        td { "Estas olda" } }
                    tr { td { [icons::NOT_OLD] }    td { "Estas nova" } }
                    tr { td { [icons::REVISED] }    td { "Retradukita" } }
                }
                table {
                    tr { td { [icons::CHRISTMAS] }  td { "Por Kristnasko" } }
                    tr { td { [icons::HALLOWEEN] }  td { "Por Haloveno" } }
                    tr { td { [icons::NEW_YEARS] }  td { "Por Novjaron" } }
                }
            }
            table ."graph" {
                [:for post in posts.into_iter().rev() { [:where let post = post.get(); {
                        tr {
                            td { @post_special_icon [post.special] }

                            td { [:if post.props.good { [icons::GOOD] }] }

                            td { a [href=url!(post.index()), title=post.title] {
                                [:if post.is_sunday
                                     { b { [&post.index()] } }
                                    else { [&post.index()] }
                                ]
                            }}

                            td { [:if post.transcript.is_some() { [icons::TRANSCRIPT] }] }

                            td { [:if post.is_old { [icons::OLD] } else { [icons::NOT_OLD] }] }
                            td { [:if post.is_revised { span { [icons::REVISED] } }] }
                        }
                    }]
                }]
            }
        }
    }}
}

pub fn at_grid(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base[
        "Krado",
        None,
        posts,
    ] {
        hr/
        div ."grid" {
            [:for day in ["Lundo", "Marto", "Merkredo", "Ĵaŭdo", "Vendredo", "Sabato", "Dimanĉo"] {
                div ."item day" {
                    b { [day] }
                }
            }]

            [:for post in posts {
                [:where
                    let post = post.get();
                    let name = format!("[{}] {}", post.index(), post.title);
                {
                    div ."item" {
                        a [
                            href=url!(post.index()),
                            title=post.title,
                        ] {
                            img [
                                alt=name,
                                src=assets_url!(format!("posts/{}/esperanto.png", &post.index)),
                                width=120,
                                height=120,
                            ]/
                        }
                    }
                }]
            }]
        }
    } }
}
