// Copyright 2023 RinChanNOWWW
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::File;

use anyhow::Error;
use blooming::ByrbtRSSContent;
use blooming::Item;
use blooming::MikanRSSContent;
use blooming::Result;
use blooming::TjuptRSSContent;
use chrono::DateTime;
use chrono::Local;

#[test]
fn test_parse_mikan() -> Result<()> {
    let file = File::open("tests/it/testdata/mikan.xml")?;
    let content: MikanRSSContent = yaserde::de::from_reader(file).map_err(Error::msg)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();

    let item1 = Item {
        title: "【豌豆字幕组】[海盗战记 / 冰海战记 第二季 / Vinland_Saga_S2][03][简体][1080P][MP4]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc3339("2023-01-24T14:34:31.721+08:00")
            .unwrap()
            .with_timezone(&Local {}),
        url:
            "https://mikanani.me/Download/20230124/5dd79686d9b6c1ab2a6091363d493d05333d8899.torrent"
                .to_string(),
    };
    let item2 = Item {
        title: "[ANi] The Vampire Dies in No Time S2 - 吸血鬼马上死 第二季 - 03".to_string(),
        pub_date: DateTime::parse_from_rfc3339("2023-01-23T21:37:12.436+08:00")
            .unwrap()
            .with_timezone(&Local {}),
        url:
            "https://mikanani.me/Download/20230123/fa2fca2b18dc4d6e166cab56fd36dcb547eafe6e.torrent"
                .to_string(),
    };
    let expected = vec![item1, item2];

    assert_eq!(items, expected);

    Ok(())
}

#[test]
fn test_parse_byrbt() -> Result<()> {
    let file = File::open("tests/it/testdata/byrbt.xml")?;
    let content: ByrbtRSSContent = yaserde::de::from_reader(file).map_err(Error::msg)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();

    let expected = vec![
        Item {
            title: "[大陆][三体][Three-Body.S01.2023.WEB-DL.4K.H265.AAC-venti][S01E12][MP4]"
                .to_string(),
            pub_date: DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 21:29:39 +0800")
                .unwrap()
                .with_timezone(&Local {}),
            url: "https://byr.pt/details.php?id=330667".to_string(),
        },
        Item {
            title: "[大陆][三体][Three.Body.S01.2023.2160p.DV.WEB-DL.H265.DDP5.1.Atmos-CHDWEB]"
                .to_string(),
            pub_date: DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 20:21:43 +0800")
                .unwrap()
                .with_timezone(&Local {}),
            url: "https://byr.pt/details.php?id=330666".to_string(),
        },
    ];

    assert_eq!(items, expected);

    Ok(())
}

#[test]
fn test_parse_tjubt() -> Result<()> {
    let file = File::open("tests/it/testdata/tjupt.xml")?;
    let content: TjuptRSSContent = yaserde::de::from_reader(file).map_err(Error::msg)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();
    let expected = vec![Item {
        title: "[Amarcord.1973.Criterion.Collection.1080p.BluRay.x264-WiKi]".to_string(),
        pub_date: DateTime::parse_from_rfc2822("Mon, 03 May 2021 05:35:49 +0000")
            .unwrap()
            .with_timezone(&Local {}),
        url: "https://www.tjupt.org/details.php?id=242844&hit=1".to_string(),
    }];

    assert_eq!(items, expected);

    Ok(())
}
