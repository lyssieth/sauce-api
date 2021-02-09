mod common;

use common::*;

fn get_source() -> IQDB {
    IQDB
}

// Warning: Some of these links may be explicit or NSFW.
// I would recommend not going to these unless you are okay with that.
static COMPARE_RESULTS: &[TestItem] = &[
    TestItem {
        link: "https://anime-pictures.net/pictures/view_post/262043?lang=en",
        similarity: 44.0,
    },
    TestItem {
        link: "https://anime-pictures.net/pictures/view_post/526757?lang=en",
        similarity: 43.0,
    },
    TestItem {
        link: "http://e-shuushuu.net/image/770593/",
        similarity: 43.0,
    },
    TestItem {
        link: "https://anime-pictures.net/pictures/view_post/262531?lang=en",
        similarity: 43.0,
    },
    TestItem {
        link: "https://yande.re/post/show/331667",
        similarity: 43.0,
    },
    TestItem {
        link: "https://yande.re/post/show/671613",
        similarity: 42.0,
    },
    TestItem {
        link: "https://konachan.com/post/show/94763",
        similarity: 41.0,
    },
    TestItem {
        link: "http://www.zerochan.net/3164063",
        similarity: 41.0,
    },
    TestItem {
        link: "http://e-shuushuu.net/image/268062/",
        similarity: 41.0,
    },
    TestItem {
        link: "http://www.zerochan.net/2104273",
        similarity: 41.0,
    },
    TestItem {
        link: "https://danbooru.donmai.us/posts/4075639",
        similarity: 41.0,
    },
    TestItem {
        link: "http://e-shuushuu.net/image/923720/",
        similarity: 41.0,
    },
    TestItem {
        link: "https://chan.sankakucomplex.com/post/show/6119972",
        similarity: 41.0,
    },
    TestItem {
        link: "https://danbooru.donmai.us/posts/2129443",
        similarity: 41.0,
    },
    TestItem {
        link: "https://danbooru.donmai.us/posts/2754047",
        similarity: 41.0,
    },
    TestItem {
        link: "https://gelbooru.com/index.php?page=post&s=view&id=805237",
        similarity: 41.0,
    },
];

#[crate::test]
async fn build_url_test() {
    let src = get_source();

    let res = src.build_url(TEST_URL).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(
        res.as_str(),
        "https://iqdb.org/?url=https://i.imgur.com/vRsNUMS.jpg"
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
