mod common;

use common::*;

fn get_source() -> Yandex {
    Yandex
}

// Yandex integration tests are *extremely* likely to be hit by a captcha wall.
// And I don't want to start solving their captchas automatically, so if that
// does occur, we simply pass the test, because we can't do anything about that.
// This is probably the biggest issue currently.

// Warning: Some of these links may be explicit or NSFW.
// I would recommend not going to these unless you are okay with that.
static COMPARE_RESULTS: &[TestItem] = &[
    TestItem {
        link: "https://anupghosal.com/wp-content/uploads/2020/09/art-anime-girl-cute-wallpapers-hd-apk-cute-anime-pictures-2048x2048.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "http://pm1.narvii.com/7132/3880fb5c9c86a59dcca45f76af475cc7a9393af7r1-1454-2016v2_uhq.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://cutee.net/wp-content/uploads/2020/09/3196-9.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://image.winudf.com/v2/image/Y29tLmFuZHJvbW8uZGV2NjYwNjE0LmFwcDczOTEwN19zY3JlZW5fM18xNTE3NzE0MjE3XzAyNA/screen-3.jpg?fakeurl=1&type=.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://static6.hentai-img.com/upload/20180530/453/463217/33.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://firebasestorage.googleapis.com/v0/b/mambet-storage/o/Channel%2FAvatar%2F24659933%2F24659933-b29026e2-779c-42af-96e3-40181c4d85e7?alt=media&token=71b97d64-0f99-4512-9533-941e6bfa76f9",
        similarity: -1.0,
    },
    TestItem {
        link: "https://em.wattpad.com/e935def3b6b8965019e121b634af2e08d7710d0c/68747470733a2f2f73332e616d617a6f6e6177732e636f6d2f776174747061642d6d656469612d736572766963652f53746f7279496d6167652f37316b3559454939666e632d42413d3d2d3631343934393039392e313534383832653239313431343931633135343737343635303539322e6a7067",
        similarity: -1.0,
    },
    TestItem {
        link: "https://yt3.ggpht.com/a/AATXAJxTfrT9L8_YAvsmqmmElPKLBKrqSY0nzg3E6fmF=s900-c-k-c0xffffffff-no-rj-mo",
        similarity: -1.0,
    },
    TestItem {
        link: "https://cutewallpaper.org/21/animation-girl-photo/Cute-Anime-Girl-Wallpaper-for-Android-APK-Download.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://1.bp.blogspot.com/-JxNB_mHRToI/X8qCnvPWMKI/AAAAAAAARz4/zjtdtBv3M9o-V29lwhMhH-ql44W3_CerwCLcBGAsYHQ/s1008/%25D8%25AA%25D9%2586%25D8%25B2%25D9%258A%25D9%2584-%25D8%25B5%25D9%2588%25D8%25B1-%25D8%25AA%25D8%25AD%25D9%2585%25D9%258A%25D9%2584-%25D8%25B5%25D9%2588%25D8%25B1%2B%252820%2529.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://sun9-42.userapi.com/WFMqizbqzly2wQoqIgxzo700DDR-IT1geqE74g/mf18CT_SIy0.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://sun9-58.userapi.com/c858220/v858220280/a12bf/cPNNdLOBrA8.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://sun9-30.userapi.com/impf/c841039/v841039578/6ff16/bTbUKE00RdE.jpg?size=453x604&quality=96&sign=dd65b0122314b56f43a62a5c3fb197ca&c_uniq_tag=UaofSaJYoTcP6bs7_riZIzCjy9zLx6R_m0YqP_oEoLk",
        similarity: -1.0,
    },
    TestItem {
        link: "https://sun9-63.userapi.com/P-VjznbD9Ytl6EPu9saBHkpBr57HxAuHYkzkPA/NEzmthg63KE.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://i1.sndcdn.com/avatars-AFVea9RZG6YIWdqS-bR8UFw-t500x500.jpg",
        similarity: -1.0,
    },
    TestItem {
        link: "https://lh3.googleusercontent.com/qyk0Q7jti6YHsNusgK5hF8oDUg0EcoX0gMGgxKCZkezGkOFrCv-AM7D3t5RGI0fSMKL_",
        similarity: -1.0,
    },
    TestItem {
        link: "https://sun1-99.userapi.com/impg/c857736/v857736911/1d0ade/y826U21pDXE.jpg?size=400x0&quality=90&crop=0,1,735,735&sign=a49ca765822684c32eeeb869ffc2dc2e&c_uniq_tag=ToxDCCAbThoX7CMxgB62A1WnQqY3X1FHFhTrqbCTO90&ava=1",
        similarity: -1.0,
    },
];

#[crate::test]
async fn build_url_test() {
    let src = get_source();

    let res = src.build_url(TEST_URL).await;

    if let Ok(_res) = res {
        // @todo: Figure out an equality test for Yandex
        // @body: The links are dynamically made, as the `cbir_id` is a variable not a constant.
    } else {
        // A hacky way to ignore captcha errors, because we can't do anything about them.
        let err = format!("{}", res.unwrap_err());

        assert!(err.contains("captcha"));
    }
}

#[crate::test]
async fn find_results() {
    let src = get_source();

    let res = src.check_sauce(TEST_URL).await;

    if let Err(e) = res {
        // A hacky way to ignore captcha errors, because we can't do anything about them.
        let err = format!("{}", e);

        assert!(dbg!(err).contains("captcha"));
        return;
    }

    let res = res.unwrap();

    assert_eq!(res.original_url.as_str(), TEST_URL);

    let items = res.items;

    // This is more complex because Yandex orders the results semi-randomly every time, and we
    // can't ensure the order as far as I can tell.
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
