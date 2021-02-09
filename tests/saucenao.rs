mod common;

use common::*;

fn get_source() -> SauceNao {
    let mut src = SauceNao::new();

    let key = env!("SAUCENAO_API_KEY");

    src.set_api_key(key.to_string());

    src
}

// Warning: Some of these links may be explicit or NSFW.
// I would recommend not going to these unless you are okay with that.
static COMPARE_RESULTS: &[TestItem] = &[
    TestItem {
        link: "https://www.pixiv.net/member_illust.php?mode=medium&illust_id=85559849",
        similarity: 94.21,
    },
    TestItem {
        link: "https://deviantart.com/view/779710303",
        similarity: 95.04,
    },
    TestItem {
        link: "https://web.archive.org/web/http://www.portalgraphics.net/pg/illust/?image_id=10360",
        similarity: 43.98,
    },
    TestItem {
        link: "https://deviantart.com/view/344356611",
        similarity: 43.95,
    },
    TestItem {
        link: "https://bcy.net/illust/detail/19198",
        similarity: 43.52,
    },
    TestItem {
        link: "https://www.artstation.com/artwork/Aq21Vo",
        similarity: 43.21,
    },
    TestItem {
        link: "https://www.pixiv.net/member_illust.php?mode=medium&illust_id=69776093",
        similarity: 42.98,
    },
    TestItem {
        link: "https://pawoo.net/@NAK_",
        similarity: 42.82,
    },
    TestItem {
        link: "https://www.pixiv.net/member_illust.php?mode=medium&illust_id=49030296",
        similarity: 42.72,
    },
    TestItem {
        link: "https://www.mangaupdates.com/series.html?id=40335",
        similarity: 42.58,
    },
    TestItem {
        link: "https://www.pixiv.net/member_illust.php?mode=medium&illust_id=16340061",
        similarity: 42.45,
    },
    TestItem {
        link: "https://danbooru.donmai.us/post/show/3930369",
        similarity: 42.34,
    },
    TestItem {
        link: "https://furrynetwork.com/artwork/1110938",
        similarity: 42.16,
    },
    TestItem {
        link: "https://seiga.nicovideo.jp/seiga/im10407458",
        similarity: 42.12,
    },
];

#[crate::test]
async fn build_url_test() {
    let src = get_source();

    let res = src.build_url(TEST_URL).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(
        res,
        format!("https://saucenao.com/search.php?url=https%3A%2F%2Fi.imgur.com%2FvRsNUMS.jpg&api_key={}", env!("SAUCENAO_API_KEY"))
    );
}

#[crate::test]
async fn find_results() {
    let src = get_source();

    let res = src.check_sauce(TEST_URL).await;

    let res = match res {
        Ok(res) => Ok(res),
        Err(e) => {
            eprintln!("{:#?}", e);
            Err(e)
        }
    };

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.original_url.as_str(), TEST_URL);

    let items = res.items;

    for expected in COMPARE_RESULTS {
        let find = items.binary_search_by_key(&expected.link, |s| s.link.as_str());
        if let Ok(find) = find {
            let result = &items[find];

            assert_eq!(result.link, expected.link);

            let error_margin = f32::EPSILON;
            assert!((result.similarity - expected.similarity).abs() < error_margin);
        }
    }
}
